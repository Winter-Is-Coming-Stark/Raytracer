pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
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

pub fn random_double(low: f64, high: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(low..high)
}

pub fn random_int(low: i32, high: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(low..high)
}

pub fn fmin(a: f64, b: f64) -> f64 {
    if a <= b {
        return a;
    }
    b
}

pub fn fmax(a: f64, b: f64) -> f64 {
    if a >= b {
        return a;
    }
    b
}
