use crate::aabb::AABB;
use crate::arrect::{XYRect, XZRect, YZRect};
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::{Hittable, HittableList, Point3, Ray};
use std::sync::Arc;

pub struct _Box {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl _Box {
    pub fn new(p0: Point3, p1: Point3, ptr: Arc<dyn Material>) -> Self {
        let mut tmp = Self {
            box_min: p0,
            box_max: p1,
            sides: HittableList::new_default(),
        };

        tmp.sides.add(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        tmp.sides.add(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));

        tmp.sides.add(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        tmp.sides.add(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));

        tmp.sides.add(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        tmp.sides.add(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            ptr.clone(),
        )));
        tmp
    }
}

impl Hittable for _Box {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.box_min, self.box_max);
        true
    }
}
