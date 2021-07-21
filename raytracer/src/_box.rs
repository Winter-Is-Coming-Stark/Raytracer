use crate::{Point3, HittableList, Hittable, Ray};
use std::rc::Rc;
use crate::material::Material;
use crate::arrect::XYRect;
use crate::hittable::HitRecord;
use crate::aabb::AABB;

pub struct _Box{
    box_min: Point3,
    box_max: Point3,
    sides: HittableList
}

impl _Box{
    pub fn new(p0: Point3,p1: Point3,ptr: Rc<dyn Material>) -> Self{
        let mut tmp = Self{
            box_min: p0,
            box_max: p1,
            sides: HittableList::new_default()
        };

        tmp.sides.add(Rc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone()
        )));
        tmp.sides.add(Rc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone()
        )));

        tmp.sides.add(Rc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone()
        )));
        tmp.sides.add(Rc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone()
        )));

        tmp.sides.add(Rc::new(XYRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone()
        )));
        tmp.sides.add(Rc::new(XYRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            ptr.clone()
        )));
        tmp
    }
}

impl Hittable for _Box{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r,t_min,t_max,rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.box_min,self.box_max);
        true
    }
}