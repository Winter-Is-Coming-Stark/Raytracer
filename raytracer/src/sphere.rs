use crate::material::Material;
use crate::Point3;
use crate::Ray;
use crate::Vec3;
use std::rc::Rc;
use crate::aabb;
use crate::aabb::AABB;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }
}

impl crate::hittable::Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut crate::hittable::HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().squared_length();
        let half_b = Vec3::dot(r.direction(), oc);
        let c = oc.squared_length() - self.radius * self.radius;
        let delta = half_b.powi(2) - a * c;
        if delta > 0.0 {
            let root = delta.sqrt();
            let mut t = (-half_b - root) / a;
            if t > t_min && t < t_max {
                rec.t = t;
                rec.p = r.at(t);
                let mut outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(&r, &mut outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            t = (-half_b + root) / a;
            if t > t_min && t < t_max {
                rec.t = t;
                rec.p = r.at(t);
                let mut outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(&r, &mut outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.radius,self.radius,self.radius),
            self.center + Vec3::new(self.radius,self.radius,self.radius),
        );
        true
    }
}
