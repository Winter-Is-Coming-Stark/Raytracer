extern crate rand;

mod Color;
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

#[allow(clippy::float_cmp)]
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use crate::camera::Camera;
use crate::hittable::hit_record;
use crate::material::{Metal, Material};
use crate::material::{Dielectric, Lambertian};
use crate::rtweekend::infinity;
pub use crate::vec3::color;
use crate::vec3::Point3;
pub use hittable::Hittable;
pub use hittable_list::Hittable_list;
use image::flat::NormalForm::ColumnMajorPacked;
use rand::Rng;
pub use ray::Ray;
use rtweekend::pi;
use sphere::Sphere;
use std::collections::hash_map::Entry::Vacant;
use std::rc::Rc;
pub use vec3::Vec3;

fn ray_color(r: Ray, world: &Hittable_list, depth: i32) -> color {
    let mut rec = hit_record::new();

    if depth <= 0 {
        return color::new(0.0, 0.0, 0.0);
    }
    if world.hit(r, 0.001, infinity, &mut rec) {
        //let target = rec.p + rec.normal + Vec3::random_unit_vector();
        //return ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
        let mut scattered = Ray::default_new();
        let mut attenuation = color::zero();
        if rec
            .mat_ptr
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return ray_color(scattered, world, depth - 1) * attenuation;
        }
        color::zero();
    }

    let unit_direction = r.direction().unit();
    let t = (unit_direction.y + 1.0) * 0.5;
    return color::new(1.0, 1.0, 1.0) * (1.0 - t) + color::new(0.5, 0.7, 1.0) * t;
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().squared_length();
    let half_b = Vec3::dot(oc, r.direction());
    let c = oc.squared_length() - radius * radius;
    let delta = half_b * half_b - a * c;

    if delta < 0.0 {
        return -1.0;
    }

    return (-half_b - delta.sqrt()) / a;
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);
    //image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: f64 = 400.0;
    let image_height: f64 = image_width / aspect_ratio;
    let samples_per_pixel = 100.0;
    let max_depth = 50;

    /*
    let mut world = Hittable_list::new_default();

    let material_ground: Rc<Lambertian> = Rc::new(Lambertian::new(color::new(0.8, 0.8, 0.0)));
    let material_center: Rc<Lambertian> = Rc::new(Lambertian::new(color::new(0.1, 0.2, 0.5)));
    let material_left: Rc<Dielectric> = Rc::new(Dielectric::new(1.5));
    let material_right: Rc<Metal> = Rc::new(Metal::new(color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));
    */
    let world = random_scene();
    //camera
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let lookfrom = Point3::new(13.0,2.0,3.0);
    let lookat = Point3::new(0.0,0.0,0.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus
    );

    let origin = Point3::zero();
    let horizon = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizon / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //rand
    let mut rng = rand::thread_rng();

    //render
    println!("P3\n{} {} \n255\n", image_width, image_height);
    let mut j = image_height - 1.0;
    let mut j1 = (image_height - 1.0) as u32;
    let mut i1 = 0;
    while j >= 0.0 {
        j1 = j as u32;
        let mut i = 0.0;
        while i < image_width {
            i1 = i as u32;
            let mut s = 0.0;
            let mut pixel_color = color::new(0.0, 0.0, 0.0);
            while s < samples_per_pixel {
                let u = (i + rng.gen::<f64>()) / (image_width - 1.0);
                let v = (j + rng.gen::<f64>()) / (image_height - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
                s = s + 1.0;
            }

            let pixel = img.get_pixel_mut(j1,i1);
            let colorx = pixel_color.x as u8;
            let colory = pixel_color.y as u8;
            let colorz = pixel_color.z as u8;
            color::wrt_color(&pixel_color, samples_per_pixel);
            *pixel = image::Rgb([colorx,colory,colorz]);
            i = i + 1.0;
        }
        bar.inc(1);
        j = j - 1.0;
    }
    img.save("output/test.png").unwrap();
    bar.finish();
}

pub fn random_scene() -> Hittable_list{
    let mut world = Hittable_list::new_default();

    let ground_material: Rc<Lambertian> = Rc::new(Lambertian::new(color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    )));

    let mut rng = rand::thread_rng();

    let mut a = -11.0;
    while a < 11.0{
        let mut b = -11.0;
        while b < 11.0{
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(a + 0.9 * rng.gen::<f64>(),0.2,b + 0.9 * rng.gen::<f64>());

            if (center - Point3::new(4.0,0.2,0.0)).length() > 0.9{
                if choose_mat < 0.8{
                    let albedo = color::random() * color::random();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                }

                else if choose_mat < 0.95{
                    let albedo = color::random_range(0.5,1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Rc::new(Metal::new(albedo,fuzz));
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                }

                else{
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                }
            }
            b = b + 1.0;
        }
        a = a + 1.0;
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0,1.0,0.0),
        1.0,
        material1.clone(),
    )));

    let material2 = Rc::new(Lambertian::new(color::new(0.4,0.2,0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0,1.0,0.0),
        1.0,
        material2.clone(),
    )));

    let material3 = Rc::new(Metal::new(color::new(0.7,0.6,0.5),0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0,1.0,0.0),
        1.0,
        material3.clone(),
    )));
    world
}
