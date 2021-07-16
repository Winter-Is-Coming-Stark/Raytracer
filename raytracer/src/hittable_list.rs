use crate::hittable::hit_record;
use crate::hittable::Hittable;
use crate::Ray;
use std::rc::Rc;
use std::vec;

#[derive(Clone)]
pub struct Hittable_list {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl Hittable_list {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn new(object: Rc<dyn Hittable>) -> Hittable_list {
        let mut objectsTmp: Vec<Rc<dyn Hittable>> = Vec::new();
        objectsTmp.push(object);
        Hittable_list {
            objects: objectsTmp,
        }
    }

    pub fn new_default() -> Hittable_list {
        let mut objectsTmp: Vec<Rc<dyn Hittable>> = Vec::new();
        Hittable_list {
            objects: objectsTmp,
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl crate::hittable::Hittable for Hittable_list {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        let mut tmp_rec: hit_record = hit_record::new();
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
