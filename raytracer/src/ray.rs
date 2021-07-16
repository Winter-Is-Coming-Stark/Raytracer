use crate::vec3::color;
use crate::vec3::Point3;
use crate::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: Point3::copy(origin),
            dir: Vec3::copy(direction),
        }
    }

    pub fn default_new() -> Self {
        Ray {
            orig: Point3::zero(),
            dir: Vec3::zero(),
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
}
