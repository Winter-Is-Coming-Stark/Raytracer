use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::Ray;
use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}
unsafe impl Sync for HittableList {}
unsafe impl Send for HittableList {}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn new(object: Arc<dyn Hittable>) -> HittableList {
        let objects_tmp: Vec<Arc<dyn Hittable>> = vec![object];

        HittableList {
            objects: objects_tmp,
        }
    }

    pub fn new_default() -> HittableList {
        let objects_tmp: Vec<Arc<dyn Hittable>> = Vec::new();
        HittableList {
            objects: objects_tmp,
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
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box: AABB = AABB::default_new();
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                AABB::surrounding_box(*output_box, temp_box)
            };
            first_box = true;
        }
        true
    }
}
