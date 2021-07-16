use crate::material::{Lambertian, Material};
use crate::Ray;
use crate::Vec3;
use crate::{color, Point3};
use std::rc::Rc;

#[derive(Clone)]
pub struct hit_record {
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl hit_record {
    pub fn new() -> hit_record {
        hit_record {
            p: Point3::zero(),
            normal: Vec3::zero(),
            front_face: false,
            t: 0.0,
            mat_ptr: Rc::new(Lambertian::new(color::new(0.0, 0.0, 0.0))),
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &mut Vec3) {
        self.front_face = Vec3::dot(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool;
}
