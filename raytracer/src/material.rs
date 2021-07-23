use crate::hittable::HitRecord;
use crate::texture::SolidColor;
use crate::texture::Texture;
use crate::vec3::Color;
use crate::Ray;
use crate::Vec3;
use rand::Rng;
use std::sync::Arc;

pub trait Material {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

//Lambertian
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }

    pub fn new_by_pointer(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}

//metal
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Metal {
        let mut f1 = f;
        if f1 > 1.0 {
            f1 = 1.0
        }
        Metal {
            albedo: a,
            fuzz: f1,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction().unit(), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            r_in.time(),
        );
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

//dielectric
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(i: f64) -> Dielectric {
        Dielectric { ir: i }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::ones();
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction().unit();

        let mut cos_theta = Vec3::dot(-unit_direction, rec.normal);
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction;
        let mut rng = rand::thread_rng();
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
        {
            direction = Vec3::reflect(unit_direction, rec.normal);
        } else {
            direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
        }
        *scattered = Ray::new(rec.p, direction, r_in.time());
        true
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new_by_color(c: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u_: f64, v_: f64, p_: &Vec3) -> Color {
        self.emit.value(u_, v_, *p_)
    }
}

//Isotropic
pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new_by_color(c: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}
