use crate::material::{Lambertian, Material};
use crate::Ray;
use crate::Vec3;
use crate::{Color, Point3};
use std::rc::Rc;
use crate::aabb::AABB;
use crate::rtweekend::{degrees_to_radians, INFINITY, fmin, fmax};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f64,
    pub mat_ptr: Rc<dyn Material>,
    pub u: f64,
    pub v: f64
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::zero(),
            normal: Vec3::zero(),
            front_face: false,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            mat_ptr: Rc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
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

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self,time0: f64,time1: f64,output_box: &mut AABB) ->bool;
}

//Translate
pub struct Translate{
    ptr: Rc<dyn Hittable>,
    offset: Vec3
}

impl Translate{
    pub fn new(p: Rc<dyn Hittable>,displacement: Vec3) -> Self{
        Self{
            ptr: p,
            offset: displacement
        }
    }
}

impl Hittable for Translate{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut moved_r = Ray::new(r.origin() - self.offset,r.direction(),r.time());
        if !self.ptr.hit(moved_r,t_min,t_max,rec){
            return false;
        }
        rec.p += self.offset;
        let mut outward_normal = rec.normal;
        rec.set_face_normal(&moved_r,&mut outward_normal);
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if !self.ptr.bounding_box(time0,time1,output_box){
            return false;
        }
        *output_box = AABB::new(
            output_box.min() + self.offset,
            output_box.max() + self.offset
        );
        true
    }
}

//RotateY
pub struct RotateY{
    ptr: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB
}

impl RotateY{
    pub fn new(p: Rc<dyn Hittable>,angle: f64) -> Self{
        let radians = degrees_to_radians(angle);
        let mut tmp = Self{
            ptr: p,
            sin_theta: 0.0,
            cos_theta: 0.0,
            hasbox: true,
            bbox: AABB::default_new()
        };
        tmp.sin_theta = radians.sin();
        tmp.cos_theta = radians.cos();
        tmp.hasbox = tmp.ptr.bounding_box(0.0,1.0,&mut tmp.bbox);

        let mut min = Point3::new(INFINITY,INFINITY,INFINITY);
        let mut max = Point3::new(-INFINITY,-INFINITY,-INFINITY);

        for i_ in 0..2{
            for j_ in 0..2{
                for k_ in 0..2{
                    let x_ = i_ as f64 * tmp.bbox.max().x + (1.0 - i_ as f64) * tmp.bbox.min().x;
                    let y_ = j_ as f64 * tmp.bbox.max().y + (1.0 - j_ as f64) * tmp.bbox.min().y;
                    let z_ = k_ as f64 * tmp.bbox.max().z + (1.0 - k_ as f64) * tmp.bbox.min().z;

                    let newx = tmp.cos_theta * x_ + tmp.sin_theta * z_;
                    let newz = tmp.cos_theta * z_ - tmp.sin_theta * x_;

                    let tester = Vec3::new(newx,y_,newz);

                    min.x = fmin(min.x,tester.x);
                    min.y = fmin(min.y,tester.y);
                    min.z = fmin(min.z,tester.z);

                    max.x = fmax(max.x,tester.x);
                    max.y = fmax(max.y,tester.y);
                    max.z = fmax(max.z,tester.z);
                }
            }
        }

        tmp
    }
}

impl Hittable for RotateY{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin.x = self.cos_theta * r.origin().x - self.sin_theta * r.origin().z;
        origin.z = self.sin_theta * r.origin().x + self.cos_theta * r.origin().z;

        direction.x = self.cos_theta * r.direction().x - self.sin_theta * r.direction().z;
        direction.z = self.sin_theta * r.direction().x + self.cos_theta * r.direction().z;

        let rotated_r = Ray::new(origin,direction,r.time());

        if !self.ptr.hit(rotated_r,t_min,t_max,rec){
            return false;
        }

        let mut p_ = rec.p;
        let mut normal = rec.normal;

        p_.x = self.cos_theta * p_.x + self.sin_theta * p_.z;
        p_.z = -self.sin_theta * p_.x + self.sin_theta * p_.z;

        normal.x = self.cos_theta * normal.x + self.sin_theta * p_.z;
        normal.z = -self.sin_theta * normal.x + self.cos_theta * p_.z;

        rec.p = p_;
        rec.set_face_normal(&rotated_r,&mut normal);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }
}