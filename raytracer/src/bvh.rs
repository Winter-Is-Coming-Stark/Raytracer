use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::rtweekend::random_int;
use crate::{Hittable, HittableList, Ray};
use std::cmp::Ordering::{Greater, Less};
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    _box: AABB,
}

impl BvhNode {
    pub fn new(
        src_objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = random_int(0, 3);
        let comparator = match axis {
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            _ => BvhNode::box_z_compare,
        };
        let object_span = end - start;
        let left_tmp: Arc<dyn Hittable>;
        let right_tmp: Arc<dyn Hittable>;
        //let mut objects = src_objects.clone();

        if object_span == 1 {
            left_tmp = src_objects[start].clone();
            right_tmp = src_objects[start].clone();
        } else if object_span == 2 {
            if comparator(&src_objects[start], &src_objects[start + 1]) == Less {
                left_tmp = src_objects[start].clone();
                right_tmp = src_objects[start + 1].clone();
            } else {
                left_tmp = src_objects[start + 1].clone();
                right_tmp = src_objects[start].clone();
            }
        } else {
            src_objects.as_mut_slice()[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            left_tmp = Arc::new(BvhNode::new(src_objects, start, mid, time0, time1));
            right_tmp = Arc::new(BvhNode::new(src_objects, mid, end, time0, time1));
        }

        let mut box_left = AABB::default_new();
        let mut box_right = AABB::default_new();

        if !left_tmp.bounding_box(time0, time1, &mut box_left)
            || !right_tmp.bounding_box(time0, time1, &mut box_right)
        {
            panic!("No bounding box in bvh_node constructor")
        }
        let box_tmp = AABB::surrounding_box(box_left, box_right);
        BvhNode {
            left: left_tmp,
            right: right_tmp,
            _box: box_tmp,
        }
    }

    pub fn new_(list: &mut HittableList, time0: f64, time1: f64) -> Self {
        let tmp = list.objects.len();
        BvhNode::new(&mut list.objects, 0, tmp, time0, time1)
    }

    pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: i32) -> bool {
        let mut boxa = AABB::default_new();
        let mut boxb = AABB::default_new();
        if !a.bounding_box(0.0, 0.0, &mut boxa) || !b.bounding_box(0.0, 0.0, &mut boxb) {
            panic!("No bounding box");
        }
        if axis == 0 {
            return boxa.min().x < boxb.min().x;
        } else if axis == 1 {
            return boxa.min().y < boxb.min().y;
        } else if axis == 2 {
            return boxa.min().z < boxb.min().z;
        }
        panic!("axis out of bound")
    }

    pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        if BvhNode::box_compare(a, b, 0) {
            return Less;
        }
        Greater
    }

    pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        if BvhNode::box_compare(a, b, 1) {
            return Less;
        }
        Greater
    }

    pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        if BvhNode::box_compare(a, b, 2) {
            return Less;
        }
        Greater
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self._box.hit(r, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let maxi = if hit_left { rec.t } else { t_max };
        let hit_right = self.right.hit(r, t_min, maxi, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self._box;
        true
    }
}
