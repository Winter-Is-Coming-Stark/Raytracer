extern crate rand;

mod _box;
mod aabb;
mod arrect;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use crate::_box::_Box;
use crate::arrect::{XZRect, YZRect, XYRect};
use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{HitRecord, RotateY, Translate};
use crate::material::{Dielectric, Lambertian};
use crate::material::{DiffuseLight, Metal};
use crate::moving_sphere::MovingSphere;
use crate::rtweekend::INFINITY;
use crate::rtweekend::{clamp, random_double};
use crate::texture::ImageTexture;
use crate::texture::NoiseTexture;
pub use crate::vec3::Color;
use crate::vec3::Point3;
pub use hittable::Hittable;
pub use hittable_list::HittableList;
use rand::Rng;
pub use ray::Ray;
use sphere::Sphere;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;
pub use vec3::Vec3;
use std::rc::Rc;
use crate::texture::CheckerTexture;
use imageproc::distance_transform::Norm::L1;

fn ray_color(r: Ray, background: Color, world: &Arc<HittableList>, depth: i32) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return background;
    }

    let mut scattered = Ray::default_new();
    let mut attenuation = Color::zero();
    let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
    let tmp_rec = rec.clone();
    if !rec
        .mat_ptr
        .scatter(r, &tmp_rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }
    emitted + ray_color(scattered, background, world, depth - 1) * attenuation
}

fn main() {
    //image
    let aspect_ratio = 1.0;
    let image_width: f64 = 800.0;
    let image_height: f64 = image_width / aspect_ratio;
    let samples_per_pixel = 5000.0;
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

    let world = my_scene();
    let background = Vec3::new(0.0, 0.0, 0.0);

    //camera
    let lookfrom = Point3::new(30.0, 0.0, 50.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let cam = Camera::new(
        (lookfrom, lookat),
        vup,
        55.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        (0.0, 1.0),
    );

    //render
    let is_ci = match std::env::var("CI") {
        Ok(x) => x == "true",
        Err(_) => false,
    };

    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 4) } else { (16, 2) };

    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );

    let (tx, rx) = channel();

    let pool = ThreadPool::new(n_workers);

    let bar = ProgressBar::new(n_jobs as u64);

    let world = Arc::new(world);

    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ptr = world.clone();
        let cam_ptr = cam.clone();

        pool.execute(move || {
            let mut rng = rand::thread_rng();
            let row_begin = image_height as usize * i / n_jobs;
            let row_end = image_height as usize * (i + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(image_width as u32, render_height as u32);

            for x in 0..image_width as i32 {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    //color_calculate

                    let mut s_ = 0.0;
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    while s_ < samples_per_pixel {
                        let u_ = (x as f64 + rng.gen::<f64>()) / (image_width - 1.0);
                        let v_ = (y as f64 + rng.gen::<f64>()) / (image_height - 1.0);
                        let r_ = cam_ptr.get_ray(u_, v_);
                        pixel_color += ray_color(r_, background, &world_ptr, max_depth);
                        s_ += 1.0;
                    }

                    //color_write
                    let pixel = img.get_pixel_mut(x as u32, img_y as u32);

                    let mut r_ = pixel_color.x;
                    let mut g_ = pixel_color.y;
                    let mut b_ = pixel_color.z;
                    let scale = 1.0 / samples_per_pixel;
                    r_ = (scale * r_).sqrt();
                    g_ = (scale * g_).sqrt();
                    b_ = (scale * b_).sqrt();
                    r_ = rtweekend::clamp(r_, 0.0, 0.999);
                    g_ = rtweekend::clamp(g_, 0.0, 0.999);
                    b_ = rtweekend::clamp(b_, 0.0, 0.999);
                    let r_ = r_ * 255.999;
                    let g_ = g_ * 255.999;
                    let b_ = b_ * 255.999;
                    let r_ = r_ as i64;
                    let g_ = g_ as i64;
                    let b_ = b_ as i64;

                    *pixel = image::Rgb([r_ as u8, g_ as u8, b_ as u8]);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }

    /*
    let mut img: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);

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
                pixel_color += ray_color(r_, background, &world, max_depth);
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
    img.save("test.png").unwrap();
    bar.finish();
     */

    let mut img: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);

    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..image_width as i32 {
                let row = row as u32;
                let idx = idx as u32;
                *img.get_pixel_mut(col as u32, row) = *data.get_pixel(col as u32, idx);
            }
        }
        bar.inc(1);
    }

    img.save("output/test.jpg").unwrap();
    bar.finish();
}

/*
pub fn random_scene() -> BvhNode {
    let mut world = HittableList::new_default();
    let checker = Arc::new(CheckerTexture::new_by_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_by_pointer(checker)),
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
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
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
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
            b += 1.0;
        }
        a += 1.0;
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    BvhNode::new_(&mut world, 0.0, 1.0)
}

fn two_spheres() -> BvhNode {
    let mut objects = HittableList::new_default();

    let checker = Arc::new(CheckerTexture::new_by_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_by_pointer(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_by_pointer(checker)),
    )));
    BvhNode::new_(&mut objects, 0.0, 0.0)
}

fn two_perlin_spheres() -> BvhNode {
    let mut objects = HittableList::new_default();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_by_pointer(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_by_pointer(pertext)),
    )));
    BvhNode::new_(&mut objects, 0.0, 0.0)
}
*/
fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_by_pointer(earth_texture.clone()));
    let mut objects = HittableList::new_default();
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface.clone(),
    )));
    objects
}

fn simple_light() -> BvhNode {
    let mut objects = HittableList::new_default();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_by_pointer(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_by_pointer(pertext.clone())),
    )));

    let difflight = Arc::new(DiffuseLight::new_by_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        difflight.clone(),
    )));
    BvhNode::new_(&mut objects, 0.0, 0.0)
}

fn cornell_box() -> BvhNode {
    let mut objects = HittableList::new_default();
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_by_color(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));
    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = Arc::new(_Box::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));

    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(_Box::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));

    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);

    BvhNode::new_(&mut objects, 0.0, 0.0)
}

fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new_default();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_by_color(Color::new(7.0, 7.0, 7.0)));

    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));
    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        113.0,
        443.0,
        127.0,
        432.0,
        554.0,
        light.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = Arc::new(_Box::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));

    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable> = Arc::new(_Box::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));

    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    objects.add(Arc::new(ConstantMedium::new_by_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    objects.add(Arc::new(ConstantMedium::new_by_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));
    objects
}

fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new_default();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(_Box::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )))
        }
    }

    let mut objects = HittableList::new_default();

    objects.add(Arc::new(BvhNode::new_(&mut boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new_by_color(Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new_by_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::new_by_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::new_by_pointer(Arc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    objects.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new_by_pointer(pertext)),
    )));

    let mut boxes2 = HittableList::new_default();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;

    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )))
    }
    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_(&mut boxes2, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}

fn my_scene() -> HittableList{
    let mut objects = HittableList::new_default();
    let ground_material = Arc::new(Metal::new(Color::new(0.0,0.7,0.9),0.5));

    //objects.add(Arc::new(Sphere::new(Point3::new(0.0,0.0,0.0),100.0,ground_material.clone())));
    //objects.add(Arc::new(Sphere::new(Point3::new(0.0,0.0,0.0),-90.0,ground_material)));

    let star1 = Arc::new(DiffuseLight::new_by_color1());
    objects.add(Arc::new(_Box::new(
        Point3::new(-4.0,-7.0,-7.0),
        Point3::new(10.0,7.0,7.0),
        star1.clone()
    )));

    objects.add(Arc::new(_Box::new(
        Point3::new(-6.0,-9.0,-9.0),
        Point3::new(12.0,9.0,9.0),
        Arc::new(Dielectric::new(3.0))
    )));

    let star3 = Arc::new(DiffuseLight::new_by_color3());
    objects.add(Arc::new(Sphere::new(
        Point3::new(-14.0,8.0,10.0),
        3.0,
        star3.clone()
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(15.0,7.0,-7.0),
        2.0,
        star3.clone()
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(18.0,11.0,18.0),
        4.0,
        star3.clone()
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(22.0,-13.0,15.0),
        1.5,
        star3.clone()
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(-10.0,-20.0,17.0),
        1.5,
        star3.clone()
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(17.0,-17.0,17.0),
        0.5,
        star3.clone()
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(-10.0,-9.0,-13.0),
        2.0,
        star3.clone()
    )));
    /*
    let mut fogs = HittableList::new_default();
    let ns = 100;

    for _j in 0..ns{
        let fog_sphere = Arc::new(Sphere::new(
            Point3::random_in_unit_sphere() * 5.0,
            0.3,
            star3.clone()
        ));
        fogs.add(Arc::new(ConstantMedium::new_by_color(
            fog_sphere,
            0.0001,
            Color::new(0.4,0.2,0.8)
        )))
    }
    objects.add(Arc::new(Translate::new(
        Arc::new(BvhNode::new_(
            &mut fogs,
            0.0,
            0.0
        )),
        Vec3::new(-20.0,10.0,10.0)
        )
    ));
*/

    objects.add(Arc::new(XZRect::new(
        -100.0,
        100.0,
        -100.0,
        100.0,
        10.0,
        Arc::new(Metal::new(Color::new(0.9,0.9,0.9),0.0))
    )));
    /*
    objects.add(Arc::new(YZRect::new(
        -100.0,
        100.0,
        -100.0,
        100.0,
        -18.0,
        Arc::new(Metal::new(Color::new(0.9,0.9,0.9),0.0))
    )));
*/
    objects.add(Arc::new(XYRect::new(
        -100.0,
        100.0,
        -100.0,
        100.0,
        -20.0,
        Arc::new(Metal::new(Color::new(0.9,0.9,0.9),0.0))
    )));

    let star5 = Arc::new(DiffuseLight::new_by_color5());
    let star6 = Arc::new(DiffuseLight::new_by_color6());
    objects.add(Arc::new(Sphere::new(
        Point3::new(16.0,-10.0,15.0),
        5.0,
        star5.clone()
    )));


    let mut fogs = HittableList::new_default();
    let ns = 10000;

    for _j in 0..ns{
        let fog_sphere = Arc::new(Sphere::new(
            Point3::new(16.0,-10.0,15.0) + Vec3::random_in_unit_disk().unit() * 10.0 * random_double(0.8,1.0),
            0.1,
            star6.clone()
        ));
        fogs.add(fog_sphere);
        let fog_sphere = Arc::new(Sphere::new(
            Point3::new(16.0,-10.0,15.0) + Vec3::random_in_unit_disk().unit() * 9.5 * random_double(0.7,1.0),
            0.1,
            star5.clone()
        ));
        fogs.add(fog_sphere);
        let fog_sphere = Arc::new(Sphere::new(
            Point3::new(16.0,-10.0,15.0) + Vec3::random_in_unit_disk().unit() * 8.0 * random_double(0.8,1.0),
            0.1,
            star6.clone()
        ));
    }
    objects.add(Arc::new(BvhNode::new_(
        &mut fogs,
        0.0,0.0,
    )));
    objects
}