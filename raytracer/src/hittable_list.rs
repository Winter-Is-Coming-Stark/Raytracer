use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::Ray;
use std::rc::Rc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn new(object: Rc<dyn Hittable>) -> HittableList {
        let mut objectsTmp: Vec<Rc<dyn Hittable>> = Vec::new();
        objectsTmp.push(object);
        HittableList {
            objects: objectsTmp,
        }
    }

    pub fn new_default() -> HittableList {
        let mut objectsTmp: Vec<Rc<dyn Hittable>> = Vec::new();
        HittableList {
            objects: objectsTmp,
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl crate::hittable::Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmp_rec: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if (*object).hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t.clone();
                *rec = tmp_rec.clone();
            }
        }
        hit_anything
    }
}
