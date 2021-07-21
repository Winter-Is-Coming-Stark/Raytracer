use crate::{Color, Vec3};
use crate::Point3;
use std::rc::Rc;
use crate::perlin::Perlin;
use image::GenericImageView;
use crate::rtweekend::clamp;

pub trait Texture{
    fn value(&self,u: f64,v: f64,p: Point3) -> Color;
}

//SolidColor
pub struct SolidColor{
    color_value: Color,
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

//NoiseTexture
pub struct NoiseTexture{
    noise: Perlin,
    scale: f64
}

impl NoiseTexture{
    pub fn new(sc: f64) -> Self{
        let mut tmp = Self{
            noise: Perlin::new(),
            scale: sc
        };
        tmp.noise.init();
        tmp
    }
}

impl Texture for NoiseTexture{
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Color::new(1.0,1.0,1.0) * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p,7)).sin())
    }
}

//ImageTexture
pub struct ImageTexture{
    data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32
}

impl ImageTexture{
    pub fn new(filename: &str) -> Self{
        let img = image::open(filename).unwrap();
        let buf = img.clone().into_bytes();
        let width_tmp = img.clone().width();
        let height_tmp = img.height();

        Self{
            data: buf,
            width: width_tmp,
            height: height_tmp,
            bytes_per_scanline: 3 * width_tmp
        }
    }
}

impl Texture for ImageTexture{
    fn value(&self, mut u_: f64, mut v_: f64, p_: Vec3) -> Vec3 {
        u_ = clamp(u_,0.0,1.0);
        v_ = 1.0 - clamp(v_,0.0,1.0);

        let mut i = (u_ * self.width as f64) as usize;
        let mut j = (v_ *self.height as f64) as usize;

        if i >= self.width as usize {i = (self.width - 1) as usize;}
        if j >= self.height as usize {j = (self.height - 1) as usize;}

        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * self.data[j * self.bytes_per_scanline as usize + i * 3 + 0] as f64,
            color_scale * self.data[j * self.bytes_per_scanline as usize + i * 3 + 1] as f64,
            color_scale * self.data[j * self.bytes_per_scanline as usize + i * 3 + 2] as f64,
        )
    }
}