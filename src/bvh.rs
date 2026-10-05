use std::ops::{Add, AddAssign};
use glam::DVec3;
use rand::random_range;
use crate::structs::{HitInfo, Hittable, Ray};
use crate::interval::Interval;


pub enum Bvh {
    Leaf { object: Hittable, bounding_box: BoundingBox},
    Node { left: Box<Bvh>, right: Box<Bvh>, bounding_box: BoundingBox}
}
impl Bvh {
    pub fn get_bounding_box(&self) -> BoundingBox {
        match self {
            Bvh::Leaf {bounding_box, ..} => *bounding_box,
            Bvh::Node {bounding_box, ..} => *bounding_box,
        }
    }
    pub fn hit(&self, ray:&Ray, ray_t:&mut Interval, hit_info:&mut HitInfo) -> bool {
        match self {
            Bvh::Leaf { object, .. } => object.hit(ray, ray_t, hit_info),
            Bvh::Node { left, right, bounding_box } => {
                if !bounding_box.hit(ray, ray_t) {
                    return false
                }
                let hit_left:bool = left.hit(ray, ray_t, hit_info);
                /*
                let new_max:f64 = if hit_left {
                    hit_info.t
                } else {
                    ray_t.max
                };
                let mut new_interval = Interval { min: ray_t.min, max: new_max };
                */
                let hit_right: bool = right.hit(ray, ray_t, hit_info);
                hit_left || hit_right
            }
        }
    }
    pub fn build(objects:&mut Vec<Hittable>) -> Bvh {
        if objects.len() == 1 {
            let object = objects[0].clone();
            let bounding_box:BoundingBox = object.bounding_box();
            Bvh::Leaf {object,bounding_box}
        } else {
            let mut bounding_box:BoundingBox = objects[0].bounding_box();
            for i in 1..objects.len() {
                bounding_box += objects[i].bounding_box();
            }
            let axis:u32 = random_range(1..=3);
            objects.sort_unstable_by(|a,b| {
                let a_min:f64 = a.bounding_box().axis(axis).min; // finds the left-most sphere
                let b_min:f64 = b.bounding_box().axis(axis).min;
                a_min.total_cmp(&b_min)
            });
            let middle:usize = objects.len() / 2;
            let (left_side,right_side) = objects.split_at(middle);
            let left = Bvh::build(&mut left_side.to_vec());
            let right = Bvh::build(&mut right_side.to_vec());
            Bvh::Node {
                left:Box::new(left),
                right:Box::new(right),
                bounding_box,
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct BoundingBox {
    pub interval_x:Interval,
    pub interval_y:Interval,
    pub interval_z:Interval,
}

impl Add for BoundingBox {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let interval_x:Interval = Interval::join(self.interval_x,rhs.interval_x);
        let interval_y:Interval = Interval::join(self.interval_y,rhs.interval_y);
        let interval_z:Interval = Interval::join(self.interval_z,rhs.interval_z);
        BoundingBox {
            interval_x,
            interval_y,
            interval_z
        }
    }
}

impl AddAssign for BoundingBox {
    fn add_assign(&mut self, rhs: Self) {
        let interval_x:Interval = Interval::join(self.interval_x,rhs.interval_x);
        let interval_y:Interval = Interval::join(self.interval_y,rhs.interval_y);
        let interval_z:Interval = Interval::join(self.interval_z,rhs.interval_z);
        *self = BoundingBox {
            interval_x,
            interval_y,
            interval_z
        }
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox {
            interval_x:Interval::zero(),
            interval_y:Interval::zero(),
            interval_z:Interval::zero(),
        }
    }
}

impl BoundingBox {
    pub fn create_box(a:DVec3,b:DVec3) -> Self{
        let interval_x:Interval = if a.x < b.x {Interval {min:a.x,max:b.x}} else {Interval {min:b.x,max:a.x}};
        let interval_y:Interval = if a.y < b.y {Interval {min:a.y,max:b.y}} else {Interval {min:b.y,max:a.y}};
        let interval_z:Interval = if a.z < b.z {Interval {min:a.z,max:b.z}} else {Interval {min:b.z,max:a.z}};
        BoundingBox {
            interval_x,
            interval_y,
            interval_z
        }
    }
    pub fn join_boxes(box_1:BoundingBox,box_2:BoundingBox) -> BoundingBox {
        let interval_x:Interval = Interval::join(box_1.interval_x,box_2.interval_x);
        let interval_y:Interval = Interval::join(box_1.interval_y,box_2.interval_y);
        let interval_z:Interval = Interval::join(box_1.interval_z,box_2.interval_z);
        BoundingBox {
            interval_x,
            interval_y,
            interval_z
        }
    }
    pub fn axis(&self,n:u32) -> Interval {
        match n {
            1 => self.interval_x,
            2 => self.interval_y,
            _ => self.interval_z,
        }
    }
    pub fn hit(&self,ray:&Ray,ray_t:&mut Interval) -> bool {
        let ray_origin:DVec3 = ray.origin;
        let ray_dir:DVec3 = ray.dir;
        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;
        for axis in 1..=3u32 {
            let origin_axis: f64 = match axis {
                1 => ray_origin.x,
                2 => ray_origin.y,
                _ => ray_origin.z,
            };
            let dir_axis: f64 = match axis {
                1 => ray_dir.x,
                2 => ray_dir.y,
                _ => ray_dir.z,
            };
            let axis_interval: Interval = self.axis(axis);
            let dir_inverse: f64 = 1.0 / dir_axis; // multiplication is a bit faster than division so we use the inverse dir when dividing multiple times
            let mut t0: f64 = (axis_interval.min - origin_axis) * dir_inverse;
            let mut t1: f64 = (axis_interval.max - origin_axis) * dir_inverse;
            if t1 < t0 { std::mem::swap(&mut t0, &mut t1) }
            if t0 > t_min { t_min = t0 }
            if t1 < t_max { t_max = t1 }
            if t_max <= t_min {
                return false
            }
        }
        true
    }
}
