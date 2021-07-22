use crate::rtweekend::random_int;
use crate::vec3::Point3;
use crate::Vec3;
use std::vec::Vec;

pub struct Perlin {
    point_count: i32,
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn perlin_generate_perm(&mut self, axis: i32) {
        let tmp = match axis {
            0 => &mut self.perm_x,
            1 => &mut self.perm_y,
            _ => &mut self.perm_z,
        };

        for i in 0..self.point_count {
            tmp.push(i);
        }
        Perlin::permut(tmp, self.point_count);
    }

    pub fn permut(p: &mut Vec<i32>, n: i32) {
        for i in 1..n {
            let target = random_int(0, n - i) as usize;
            let j = i as usize;
            let tmp = p[j];
            p.swap(j,target);
        }
    }

    pub fn new() -> Self {
        Self {
            point_count: 256,
            ranvec: Vec::new(),
            perm_x: Vec::new(),
            perm_y: Vec::new(),
            perm_z: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        for _i in 0..self.point_count {
            self.ranvec.push(Vec3::random_range(-1.0, 1.0));
        }

        self.perlin_generate_perm(0);
        self.perlin_generate_perm(1);
        self.perlin_generate_perm(2);
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let mut _u = p.x - p.x.floor();
        let mut _v = p.y - p.y.floor();
        let mut _w = p.z - p.z.floor();

        let _i = p.x.floor();
        let _j = p.y.floor();
        let _k = p.z.floor();

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];

        let mut di = 0.0;
        while di < 2.0 {
            let mut dj = 0.0;
            while dj < 2.0 {
                let mut dk = 0.0;
                while dk < 2.0 {
                    let id_x = (255 & (_i + di) as i32) as usize;
                    let id_y = (255 & (_j + dj) as i32) as usize;
                    let id_z = (255 & (_k + dk) as i32) as usize;
                    let tmp = (self.perm_x[id_x] ^ self.perm_y[id_y] ^ self.perm_z[id_z]) as usize;
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[tmp];
                    dk += 1.0
                }
                dj += 1.0
            }
            di += 1.0
        }
        Self::trilinear_interp(c, _u, _v, _w)
    }

    pub fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], _u: f64, _v: f64, _w: f64) -> f64 {
        let mut accum = 0.0;
        let uu = _u * _u * (3.0 - 2.0 * _u);
        let vv = _v * _v * (3.0 - 2.0 * _v);
        let ww = _w * _w * (3.0 - 2.0 * _w);

        let mut _i = 0.0;
        while _i < 2.0 {
            let mut _j = 0.0;
            while _j < 2.0 {
                let mut _k = 0.0;
                while _k < 2.0 {
                    let weight_v = Vec3::new(_u - _i, _v - _j, _w - _k);
                    accum += (_i * uu + (1.0 - _i) * (1.0 - uu))
                        * (_j * vv + (1.0 - _j) * (1.0 - vv))
                        * (_k * ww + (1.0 - _k) * (1.0 - ww))
                        * Vec3::dot(c[_i as usize][_j as usize][_k as usize], weight_v);
                    _k += 1.0;
                }
                _j += 1.0;
            }
            _i += 1.0;
        }
        accum
    }

    pub fn turb(&self, p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}
