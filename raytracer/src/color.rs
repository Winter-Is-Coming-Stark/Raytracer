use crate::rtweekend::clamp;
use crate::vec3::Color;

impl Color {
    pub fn wrt_color(pixel_color: &Color, samples_per_pixel: f64) {
        let mut r = pixel_color.x;
        let mut g = pixel_color.y;
        let mut b = pixel_color.z;
        let scale = 1.0 / samples_per_pixel;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();
        clamp(r, 0.0, 0.999);
        clamp(g, 0.0, 0.999);
        clamp(b, 0.0, 0.999);
        let r = r * 255.999;
        let g = g * 255.999;
        let b = b * 255.999;
        let r = r as i64;
        let g = g as i64;
        let b = b as i64;
        println!("{} {} {}", r, g, b);
    }
}
