extern crate rand;

mod aabb;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
mod bvh;
mod texture;
mod perlin;
mod arrect;
mod _box;
mod constant_medium;

use image::{ImageBuffer, RgbImage,ImageDecoder,GenericImageView};
use indicatif::ProgressBar;

use crate::camera::Camera;
use crate::hittable::{HitRecord, RotateY, Translate};
use crate::material::{Metal, DiffuseLight};
use crate::material::{Dielectric, Lambertian};
use crate::moving_sphere::MovingSphere;
use crate::rtweekend::INFINITY;
use crate::rtweekend::{clamp, random_double};
pub use crate::vec3::Color;
use crate::vec3::Point3;
pub use hittable::Hittable;
pub use hittable_list::HittableList;
use rand::Rng;
pub use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
pub use vec3::Vec3;
use crate::bvh::BvhNode;
use crate::texture::{CheckerTexture, ImageTexture};
use crate::texture::NoiseTexture;
use crate::arrect::{XYRect, YZRect, XZRect};
use crate::_box::_Box;
use crate::constant_medium::ConstantMedium;

fn ray_color(r: Ray, background: Color,world: &BvhNode, depth: i32) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if !world.hit(r,0.001,INFINITY,&mut rec){
        return background;
    }

    let mut scattered = Ray::default_new();
    let mut attenuation = Color::zero();
    let emitted = rec.mat_ptr.emitted(rec.u,rec.v,&rec.p);
    let mut tmp_rec = rec.clone();
    if !rec.mat_ptr.scatter(r, &mut tmp_rec,&mut attenuation,&mut scattered){
        return emitted;
    }
    emitted + ray_color(scattered,background,world,depth - 1) * attenuation
}

fn main() {
    //image
    let aspect_ratio = 1.0;
    let image_width: f64 = 600.0;
    let image_height: f64 = image_width / aspect_ratio;
    let samples_per_pixel = 200.0;
    let max_depth = 50;

    //world
    /*
    let world = random_scene();

    //camera
    let lookfrom = Point3::new(12.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    */

    let world = cornell_box();
    let background = Vec3::new(0.0,0.0,0.0);

    //camera
    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    //rand
    let mut rng = rand::thread_rng();

    //render
    println!("P3\n{} {} \n255\n", image_width, image_height);
    let mut img: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);
    let bar = ProgressBar::new(image_width as u64);

    let mut j_ = 0.0;
    while j_ < image_height {
        let mut i_ = 0.0;
        while i_ < image_width {
            let mut s_ = 0.0;
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            while s_ < samples_per_pixel {
                let u_ = (i_ + rng.gen::<f64>()) / (image_width - 1.0);
                let v_ = (j_ + rng.gen::<f64>()) / (image_height - 1.0);
                let r_ = cam.get_ray(u_, v_);
                pixel_color += ray_color(r_, background,&world, max_depth);
                s_ += 1.0;
            }
            //color write
            let pixel = img.get_pixel_mut(i_ as u32, j_ as u32);
            let mut r_ = pixel_color.x;
            let mut g_ = pixel_color.y;
            let mut b_ = pixel_color.z;
            let scale = 1.0 / samples_per_pixel;
            r_ = (scale * r_).sqrt();
            g_ = (scale * g_).sqrt();
            b_ = (scale * b_).sqrt();
            clamp(r_, 0.0, 0.999);
            clamp(g_, 0.0, 0.999);
            clamp(b_, 0.0, 0.999);
            let r_ = r_ * 255.999;
            let g_ = g_ * 255.999;
            let b_ = b_ * 255.999;
            let r_ = r_ as i64;
            let g_ = g_ as i64;
            let b_ = b_ as i64;
            //Color::wrt_color(&pixel_color, samples_per_pixel);
            *pixel = image::Rgb([r_ as u8, g_ as u8, b_ as u8]);
            i_ += 1.0;
        }
        bar.inc(1);
        j_ += 1.0;
    }
    img.save("output/test.jpg").unwrap();
    bar.finish();
}

pub fn random_scene() -> BvhNode {
    let mut world = HittableList::new_default();
    let checker = Rc::new(CheckerTexture::new_by_color(Color::new(0.2,0.3,0.1),Color::new(0.9,0.9,0.9)));

    let ground_material: Rc<Lambertian> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_by_pointer(checker)),
    )));

    let mut rng = rand::thread_rng();

    let mut a = -11.0;
    while a < 11.0 {
        let mut b = -11.0;
        while b < 11.0 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double(0.0, 0.5), 0.0);
                    world.add(Rc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
            b += 1.0;
        }
        a += 1.0;
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    BvhNode::new_(
        &world,
        0.0,
        1.0
    )
}

fn two_spheres() -> BvhNode{
    let mut objects = HittableList::new_default();

    let checker = Rc::new(CheckerTexture::new_by_color(Color::new(0.2,0.3,0.1),Color::new(0.9,0.9,0.9)));
    objects.add((Rc::new(Sphere::new(
        Vec3::new(0.0,-10.0,0.0),
        10.0,
        Rc::new(Lambertian::new_by_pointer(checker.clone()))
    ))));
    objects.add((Rc::new(Sphere::new(
        Vec3::new(0.0,10.0,0.0),
        10.0,
        Rc::new(Lambertian::new_by_pointer(checker.clone()))
    ))));
    BvhNode::new_(
        &objects,
        0.0,
        0.0
    )
}

fn two_perlin_spheres() -> BvhNode{
    let mut objects = HittableList::new_default();

    let pertext = Rc::new(NoiseTexture::new(4.0));
    objects.add((Rc::new(Sphere::new(
        Vec3::new(0.0,-1000.0,0.0),
        1000.0,
        Rc::new(Lambertian::new_by_pointer(pertext.clone()))
    ))));
    objects.add((Rc::new(Sphere::new(
        Vec3::new(0.0,2.0,0.0),
        2.0,
        Rc::new(Lambertian::new_by_pointer(pertext.clone()))
    ))));
    BvhNode::new_(
        &objects,
        0.0,
        0.0
    )
}

fn earth() -> BvhNode{
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::new_by_pointer(earth_texture.clone()));
    let mut objects = HittableList::new_default();
    objects.add((Rc::new(Sphere::new(
        Vec3::new(0.0,0.0,0.0),
        2.0,
        earth_surface.clone()
    ))));
    BvhNode::new_(
        &objects,
        0.0,
        0.0
    )
}

fn simple_light() -> BvhNode{
    let mut objects = HittableList::new_default();

    let pertext = Rc::new(NoiseTexture::new(4.0));
    objects.add((Rc::new(Sphere::new(
        Vec3::new(0.0,-1000.0,0.0),
        1000.0,
        Rc::new(Lambertian::new_by_pointer(pertext.clone()))
    ))));
    objects.add((Rc::new(Sphere::new(
        Vec3::new(0.0,2.0,0.0),
        2.0,
        Rc::new(Lambertian::new_by_pointer(pertext.clone()))
    ))));

    let difflight = Rc::new(DiffuseLight::new_by_color(Color::new(4.0,4.0,4.0)));
    objects.add(Rc::new(XYRect::new(3.0,5.0,1.0,3.0,-2.0,difflight.clone())));
    BvhNode::new_(
        &objects,
        0.0,
        0.0
    )
}

fn cornell_box() -> BvhNode{
    let mut objects = HittableList::new_default();
    let red = Rc::new(Lambertian::new(Color::new(0.65,0.05,0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73,0.73,0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12,0.45,0.15)));
    let light = Rc::new(DiffuseLight::new_by_color(Color::new(15.0,15.0,15.0)));

    objects.add(Rc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone()
    )));
    objects.add(Rc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone()
    )));
    objects.add(Rc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone()
    )));
    objects.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone()
    )));
    objects.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone()
    )));
    objects.add(Rc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone()
    )));

    let mut box1:Rc<dyn Hittable> = Rc::new(_Box::new(
        Point3::new(0.0,0.0,0.0),
        Point3::new(165.0,330.0,165.0),
        white.clone()
    ));

    box1 = Rc::new(RotateY::new(box1,15.0));
    box1 = Rc::new(Translate::new(box1,Vec3::new(265.0,0.0,295.0)));
    objects.add(box1);

    let mut box2:Rc<dyn Hittable> = Rc::new(_Box::new(
        Point3::new(0.0,0.0,0.0),
        Point3::new(165.0,165.0,165.0),
        white.clone()
    ));

    box2 = Rc::new(RotateY::new(box2,-18.0));
    box2 = Rc::new(Translate::new(box2,Vec3::new(130.0,0.0,65.0)));
    objects.add(box2);


    BvhNode::new_(
        &objects,
        0.0,
        0.0
    )
}

fn cornell_smoke() -> BvhNode{
    let mut objects = HittableList::new_default();

    let red = Rc::new(Lambertian::new(Color::new(0.65,0.05,0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73,0.73,0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12,0.45,0.15)));
    let light = Rc::new(DiffuseLight::new_by_color(Color::new(7.0,7.0,7.0)));

    objects.add(Rc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone()
    )));
    objects.add(Rc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone()
    )));
    objects.add(Rc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone()
    )));
    objects.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone()
    )));
    objects.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone()
    )));
    objects.add(Rc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone()
    )));

    let mut box1:Rc<dyn Hittable> = Rc::new(_Box::new(
        Point3::new(0.0,0.0,0.0),
        Point3::new(165.0,330.0,165.0),
        white.clone()
    ));

    box1 = Rc::new(RotateY::new(box1,15.0));
    box1 = Rc::new(Translate::new(box1,Vec3::new(265.0,0.0,295.0)));


    let mut box2:Rc<dyn Hittable> = Rc::new(_Box::new(
        Point3::new(0.0,0.0,0.0),
        Point3::new(165.0,165.0,165.0),
        white.clone()
    ));

    box2 = Rc::new(RotateY::new(box2,-18.0));
    box2 = Rc::new(Translate::new(box2,Vec3::new(130.0,0.0,65.0)));

    objects.add(Rc::new(ConstantMedium::new_by_color(box1,0.01,Color::new(0.0,0.0,0.0))));
    objects.add(Rc::new(ConstantMedium::new_by_color(box2,0.01,Color::new(1.0,1.0,1.0))));

    BvhNode::new_(
        &objects,
        0.0,
        0.0
    )
}
