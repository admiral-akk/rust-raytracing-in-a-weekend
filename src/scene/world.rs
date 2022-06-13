use super::object::Object;
use crate::{
    hittable::{bounding_box, hit_record, hit_record::HitRecord, hittable::Hittable},
    BoundingBox, Ray,
};
use rand::{thread_rng, Rng};

pub struct World {
    heap: Vec<BoundingBox>,
    objects: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        World {
            heap: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Object) {
        self.objects.push(object);
    }
}
use std::cmp::Ordering::Equal;
impl World {
    pub fn recalculate_heap(&mut self) {
        // sort the objects here
        World::sort(&mut self.objects);
        // add elements to the heap by index
        while self.heap.len() < self.objects.len() - 1 {
            self.heap.push(bounding_box::DEFAULT);
        }

        self.generate_bounding_box(0);
    }

    fn sort(objects: &mut [Object]) {
        if objects.len() <= 1 {
            return;
        }

        let mut rng = thread_rng();
        let axis = rng.gen_range(0.0..=3.0);
        if axis <= 1.0 {
            objects.sort_by(|a, b| {
                a.bounds()
                    .min
                    .x
                    .partial_cmp(&b.bounds().min.x)
                    .unwrap_or(Equal)
            });
        } else if axis <= 2.0 {
            objects.sort_by(|a, b| {
                a.bounds()
                    .min
                    .y
                    .partial_cmp(&b.bounds().min.y)
                    .unwrap_or(Equal)
            });
        } else {
            objects.sort_by(|a, b| {
                a.bounds()
                    .min
                    .y
                    .partial_cmp(&b.bounds().min.z)
                    .unwrap_or(Equal)
            });
        }
        let len = objects.len();
        let split: usize = 1 << (((objects.len() - 1) as f32).log2().floor() as usize);
        World::sort(&mut objects[0..split]);
        World::sort(&mut objects[split..len]);
    }

    fn generate_bounding_box(&mut self, index: usize) -> BoundingBox {
        if index >= self.heap.len() {
            return self.objects[index - self.heap.len()].bounds();
        }
        let left = self.generate_bounding_box(2 * index + 1);
        let right = self.generate_bounding_box(2 * index + 2);
        self.heap[index] = BoundingBox::union(&left, &right);
        return self.heap[index].bounds();
    }

    fn hit_effecient<'a>(&'a self, ray: &Ray, ret: &mut HitRecord<'a>, index: usize) {
        if index >= self.heap.len() {
            let obj_index = index - self.heap.len();
            self.test_hit(ray, &self.objects[obj_index], ret);
        } else {
            if self.heap[index].hit(ray) == f32::INFINITY {
                return;
            }
            self.hit_effecient(ray, ret, 2 * index + 1);
            self.hit_effecient(ray, ret, 2 * index + 2);
        }
    }

    fn test_hit<'a>(&'a self, ray: &Ray, object: &'a Object, ret: &mut HitRecord<'a>) {
        let t = object.hit(ray);
        if t >= ret.t {
            return;
        }
        ret.t = t;
        ret.object = Some(&object);
    }
}

impl World {
    fn hit_ineffecient<'a>(&'a self, ray: &Ray, ret: &mut HitRecord<'a>) {
        for i in 0..self.objects.len() {
            self.test_hit(ray, &self.objects[i], ret);
        }
    }
    pub fn hit<'a>(&'a self, ray: &Ray) -> HitRecord<'a> {
        let mut ret: HitRecord = hit_record::DEFAULT;
        self.hit_effecient(ray, &mut ret, 0);
        let mut ret2: HitRecord = hit_record::DEFAULT;
        //self.hit_ineffecient(ray, &mut ret2);
        // assert_eq!(ret, ret2);
        if ret.hit() {
            ret.point = ray.project(ret.t);
            ret.normal = ret.object.unwrap().hit_normal(ray, &ret.point);
        }
        return ret;
    }
}
