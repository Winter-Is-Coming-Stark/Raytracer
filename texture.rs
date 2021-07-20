use crate::{Color, Vec3};
use crate::Point3;
use std::rc::Rc;

pub trait Texture{
    fn value(&self,u: f64,v: f64,p: Point3) -> Color;
}

//SolidColor
pub struct SolidColor{
    color_value: Color
}

impl SolidColor{
    pub fn new(c: Color) -> Self{
        Self{
            color_value: c
        }
    }
    pub fn new_(red: f64,green: f64,blue: f64) -> Self{
        Self{
            color_value: Color::new(red,green,blue)
        }
    }
}

impl Texture for SolidColor{
    fn value(&self,_u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color_value
    }
}

//CheckerTexture
pub struct CheckerTexture{
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>
}

impl CheckerTexture{
    pub fn new(
        _even: Rc<dyn Texture>,
        _odd: Rc<dyn Texture>
    ) -> Self{
        Self{
            even: _even,
            odd: _odd
        }
    }

    pub fn new_by_color(
        c1: Color,
        c2: Color
    ) -> Self{
        Self{
            even: Rc::new(SolidColor::new(c1)),
            odd: Rc::new(SolidColor::new(c2))
        }
    }
}

impl Texture for CheckerTexture{
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0{
            return self.odd.value(u,v,p);
        }
        self.even.value(u,v,p)
    }
}