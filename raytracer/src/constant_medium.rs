use std::rc::Rc;
use crate::{Hittable, Color, Ray, Vec3};
use crate::material::{Material, Isotropic};
use crate::texture::Texture;
use crate::hittable::HitRecord;
use crate::aabb::AABB;
use crate::rtweekend::{INFINITY, random_double};
use rand::random;

pub struct ConstantMedium{
    boundary: Rc<dyn Hittable>,
    phase_function: Rc<dyn Material>,
    neg_inv_density: f64
}

impl ConstantMedium{
    pub fn new(b: Rc<dyn Hittable>,d: f64,a: Rc<dyn Texture>) -> Self{
        Self{
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Rc::new(Isotropic::new(a))
        }
    }
    pub fn new_by_color(b: Rc<dyn Hittable>,d: f64,c: Color) -> Self{
        Self{
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Rc::new(Isotropic::new_by_color(c))
        }
    }
}

impl Hittable for ConstantMedium{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.hit(r,-INFINITY,INFINITY,&mut rec1){
            return false;
        }
        if !self.boundary.hit(r,rec1.t + 0.0001,INFINITY,&mut rec2){
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max
        }

        if rec1.t >= rec2.t{
            return false;
        }

        if rec1.t < 0.0{
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double(0.0,1.0).ln();

        if hit_distance > distance_inside_boundary{
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0,0.0,0.0);
        rec.front_face = true;
        rec.mat_ptr = self.phase_function.clone();
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.bounding_box(time0,time1,output_box)
    }
}