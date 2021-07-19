use std::rc::Rc;
use crate::{Hittable, Ray, HittableList};
use crate::aabb::AABB;
use crate::rtweekend::random_int;
use std::panic::panic_any;
use std::slice;
use crate::hittable::HitRecord;
use std::cmp::Ordering::{Less, Greater};

pub struct BvhNode{
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    _box: AABB
}

impl BvhNode{
    pub fn new(
        src_objects: &Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64
    ) -> Self{
        let axis = random_int(0,3);
        let comparator = match axis{
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            _ => BvhNode::box_z_compare
        };
        let object_span = end - start;
        let left_tmp: Rc<dyn Hittable>;
        let right_tmp: Rc<dyn Hittable>;
        let mut objects = src_objects.clone();

        if object_span == 1{
            left_tmp = objects[start].clone();
            right_tmp = objects[start].clone();
        }
        else if object_span == 2{
            if comparator(&objects[start],&objects[start + 1]) == Less{
                left_tmp = objects[start].clone();
                right_tmp = objects[start + 1].clone();
            }
            else{
                left_tmp = objects[start + 1].clone();
                right_tmp = objects[start].clone();
            }
        }
        else{
            objects.as_mut_slice()[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            left_tmp = Rc::new(BvhNode::new(&objects,start,mid,time0,time1));
            right_tmp = Rc::new(BvhNode::new(&objects,mid,end,time0,time1));
        }

        let mut box_left = AABB::default_new();
        let mut box_right = AABB::default_new();

        if !left_tmp.bounding_box(time0,time1,&mut box_left) || !right_tmp.bounding_box(time0,time1,&mut box_right){
            panic!("No bounding box in bvh_node constructor")
        }
        let box_tmp = AABB::surrounding_box(box_left,box_right);
        BvhNode{
            left: left_tmp,
            right: right_tmp,
            _box:box_tmp
        }
    }

    pub fn new_(
        list:&HittableList,
        time0: f64,
        time1: f64
    ) -> Self{
        BvhNode::new(
            &list.objects,
            0,
            list.objects.len(),
            time0,
            time1
        )
    }

    pub fn box_compare(
        a: &Rc<dyn Hittable>,
        b: &Rc<dyn Hittable>,
        axis: i32
    ) -> bool{
        let mut boxa = AABB::default_new();
        let mut boxb = AABB::default_new();
        if !a.bounding_box(0.0,0.0,&mut boxa) || !b.bounding_box(0.0,0.0,&mut boxb){
            panic!("No bounding box");
        }
        if axis == 0 {
            return boxa.min().x < boxb.min().x;
        }
        else if axis == 1 {
            return boxa.min().y < boxb.min().y;
        }
        else if axis == 2{
            return boxa.min().z < boxb.min().z;
        }
        panic!("axis out of bound")
    }
    
    pub fn box_x_compare(a: &Rc<dyn Hittable>,b: &Rc<dyn Hittable>) -> std::cmp::Ordering{
        if BvhNode::box_compare(a,b,0){
            return Less;
        }
        Greater
    }

    pub fn box_y_compare(a: &Rc<dyn Hittable>,b: &Rc<dyn Hittable>) -> std::cmp::Ordering{
        if BvhNode::box_compare(a,b,1){
            return Less;
        }
        Greater
    }

    pub fn box_z_compare(a: &Rc<dyn Hittable>,b: &Rc<dyn Hittable>) -> std::cmp::Ordering{
        if BvhNode::box_compare(a,b,2){
            return Less;
        }
        Greater
    }
}

impl Hittable for BvhNode{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self._box.hit(r,t_min,t_max) {
            return false;
        }

        let hit_left = self.left.hit(r,t_min,t_max,rec);
        let maxi = if hit_left {rec.t} else {t_max};
        let hit_right = self.right.hit(r,t_min,maxi,rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self._box;
        true
    }
}