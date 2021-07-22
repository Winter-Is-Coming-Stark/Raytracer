use crate::aabb::AABB;
use crate::material::{Lambertian, Material};
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::HitRecord;
use crate::Vec3;
use crate::{Hittable, Ray};
use std::sync::Arc;

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        cen0: Point3,
        cen1: Point3,
        _time0: f64,
        _time1: f64,
        r: f64,
        m: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0: _time0,
            time1: _time1,
            radius: r,
            mat_ptr: m.clone(),
        }
    }

    pub fn default_new() -> Self {
        Self {
            center0: Point3::zero(),
            center1: Point3::zero(),
            time0: 0.0,
            time1: 0.0,
            radius: 0.0,
            mat_ptr: Arc::new(Lambertian::new(Color::zero())),
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + (self.center1 - self.center0) * (time - self.time0) / (self.time1 - self.time0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().squared_length();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let mut outward_normal = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(&r, &mut outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB::new(
            self.center(_time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(_time0) + Vec3::new(self.radius, self.radius, self.radius),
        );

        let box1 = AABB::new(
            self.center(_time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(_time1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        *output_box = AABB::surrounding_box(box0, box1);
        true
    }
}
