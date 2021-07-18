use crate::rtweekend::degrees_to_radians;
use crate::vec3::color;
use crate::vec3::Point3;
use crate::Ray;
use crate::Vec3;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w1 = (lookfrom - lookat).unit();
        let u1 = Vec3::cross(vup, w1).unit();
        let v1 = Vec3::cross(w1, u1);

        let hor = u1 * viewport_width * focus_dist;
        let ver = v1 * viewport_height * focus_dist;
        let llc = lookfrom - hor * 0.5 - ver * 0.5 - w1 * focus_dist;
        Camera {
            origin: lookfrom,
            horizontal: hor,
            vertical: ver,
            lower_left_corner: llc,
            u: u1,
            v: v1,
            w: w1,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
