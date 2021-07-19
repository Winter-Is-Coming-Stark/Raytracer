use crate::vec3::Point3;
use crate::Ray;
use std::cmp::{min, max};
use crate::rtweekend::fmax;
use crate::rtweekend::fmin;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn default_new() -> Self{
        Self{
            minimum: Point3::zero(),
            maximum: Point3::zero()
        }
    }

    pub fn min(&self) -> Point3{
        self.minimum
    }

    pub fn max(&self) -> Point3{
        self.maximum
    }

    pub fn hit(
        &self,
        r: Ray,
        mut t_min: f64,
        mut t_max: f64
    ) -> bool{
        let inv_d = 1.0 / r.direction().x;
        let mut t0 = (self.min().x - r.origin().x) * inv_d;
        let mut t1 = (self.max().x - r.origin().x) * inv_d;
        if inv_d < 0.0{
            let tmp = t0;
            t0 = t1;
            t1 = tmp;
        }
        t_min = if t0 > t_min {t0} else {t_min};
        t_max = if t1 < t_max {t1} else {t_max};

        if t_max <= t_min{
            return false;
        }

        let inv_d = 1.0 / r.direction().y;
        let mut t0 = (self.min().y - r.origin().y) * inv_d;
        let mut t1 = (self.max().y - r.origin().y) * inv_d;
        if inv_d < 0.0{
            let tmp = t0;
            t0 = t1;
            t1 = tmp;
        }
        t_min = if t0 > t_min {t0} else {t_min};
        t_max = if t1 < t_max {t1} else {t_max};

        if t_max <= t_min{
            return false;
        }

        let inv_d = 1.0 / r.direction().z;
        let mut t0 = (self.min().z - r.origin().z) * inv_d;
        let mut t1 = (self.max().z - r.origin().z) * inv_d;
        if inv_d < 0.0{
            let tmp = t0;
            t0 = t1;
            t1 = tmp;
        }
        t_min = if t0 > t_min {t0} else {t_min};
        t_max = if t1 < t_max {t1} else {t_max};

        if t_max <= t_min{
            return false;
        }
        true
    }

    pub fn surrounding_box(box0: AABB,box1: AABB) -> Self{
        let small = Point3::new(
            fmin(box0.min().x,box1.min().x),
            fmin(box0.min().y,box1.min().y),
            fmin(box0.min().z,box1.min().z),
        );

        let big = Point3::new(
            fmax(box0.max().x,box1.max().x),
            fmax(box0.max().y,box1.max().y),
            fmax(box0.max().z,box1.max().z),
        );

        Self::new(small,big)
    }
}
