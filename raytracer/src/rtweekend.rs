use std::rc::Rc;
use rand::Rng;

pub const infinity: f64 = f64::INFINITY;
pub const pi: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * pi / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn random_double(low : f64,high : f64) -> f64{
    let mut rng = rand::thread_rng();
    rng.gen_range(low..high)
}