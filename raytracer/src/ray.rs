use crate::vec3::Point3;
use crate::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm : f64
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3,time : f64) -> Ray {
        Ray {
            orig: Point3::copy(origin),
            dir: Vec3::copy(direction),
            tm: time
        }
    }

    pub fn default_new() -> Self {
        Ray {
            orig: Point3::zero(),
            dir: Vec3::zero(),
            tm: 0.0
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn time(&self) -> f64{self.tm}
}
