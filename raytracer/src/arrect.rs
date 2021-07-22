use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::vec3::Point3;
use crate::{Hittable, Ray, Vec3};
use std::sync::Arc;

pub struct XYRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYRect {
    pub fn new(x0_: f64, x1_: f64, y0_: f64, y1_: f64, k_: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            x0: x0_,
            x1: x1_,
            y0: y0_,
            y1: y1_,
            k: k_,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().z) / r.direction().z;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin().x + t * r.direction().x;
        let y = r.origin().y + t * r.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let mut outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(&r, &mut outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}

pub struct XZRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XZRect {
    pub fn new(x0_: f64, x1_: f64, z0_: f64, z1_: f64, k_: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            x0: x0_,
            x1: x1_,
            z0: z0_,
            z1: z1_,
            k: k_,
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().y) / r.direction().y;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin().x + t * r.direction().x;
        let z = r.origin().z + t * r.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let mut outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(&r, &mut outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }
}

pub struct YZRect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YZRect {
    pub fn new(y0_: f64, y1_: f64, z0_: f64, z1_: f64, k_: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            y0: y0_,
            y1: y1_,
            z0: z0_,
            z1: z1_,
            k: k_,
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().x) / r.direction().x;
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.origin().y + t * r.direction().y;
        let z = r.origin().z + t * r.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let mut outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(&r, &mut outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }
}
