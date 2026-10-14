use glam::DVec3;
use crate::rays::*;
use crate::structs::bvh::BoundingBox;
use crate::structs::interval::Interval;
use crate::structs::materials::Material;

#[derive(Clone,Copy)]
pub struct Quad {
    pub starting_point:DVec3,
    pub side_1:DVec3, // u
    pub side_2:DVec3, // v
    pub D:f64,
    pub w:DVec3,
    pub normal:DVec3,
    pub material: Material

}

impl Quad {
    pub fn init( starting_point:DVec3,side_1:DVec3,side_2:DVec3,material:Material) -> Self {
        let n:DVec3 = side_1.cross(side_2);
        let normal:DVec3 = n.normalize();
        let D:f64 = normal.dot(starting_point);
        let w:DVec3 = n/(n.dot(n));
        Quad {
            starting_point,
            side_1,
            side_2,
            normal,
            D,
            w,
            material
        }
    }
    pub fn bounding_box(&self) -> BoundingBox {
        let starting_point:DVec3 = self.starting_point;
        let corner_1:DVec3 = starting_point + self.side_1;
        let corner_2:DVec3 = starting_point + self.side_2;
        let corner_3:DVec3 = starting_point + self.side_1 + self.side_2;
        BoundingBox::create_box(starting_point,corner_1) + BoundingBox::create_box(starting_point,corner_2)
            + BoundingBox::create_box(starting_point,corner_3)
    }
    pub fn is_interior(a:f64,b:f64,hit_info:&mut HitInfo) -> bool {
        let unit_interval:Interval = Interval {
            min:0.0,max:1.0,
        };
        if unit_interval.contains(a) && unit_interval.contains(b) {
            hit_info.u = a;
            hit_info.v = b;
            true
        } else {
            false
        }

        /*
        if a > 0.0 && b > 0.0  && a + b < 1.0 {
            hit_info.u = a;
            hit_info.v = b;
            true
        } else {
            false
        }
        */

    }
    pub fn hit(&self,ray:&Ray,ray_t:&mut Interval,hit_info:&mut HitInfo) -> bool {
        let denom:f64 = self.normal.dot(ray.dir);
        if denom.abs() <= 1e-8 { // parallel
            return false
        }
        let t:f64 = (self.D - self.normal.dot(ray.origin))/denom;
        if !ray_t.contains(t) {return false}
        let intersection:DVec3 = ray.pos(t);
        let p:DVec3 = intersection - self.starting_point;
        let alpha:f64 = self.w.dot(p.cross(self.side_2));
        let beta:f64 = self.w.dot(self.side_1.cross(p));

        if !Self::is_interior(alpha,beta,hit_info) {
            return false;
        }

        hit_info.t = t;
        hit_info.position = intersection;
        hit_info.material = self.material;
        hit_info.did_hit = true;
        hit_info.set_normal(ray,self.normal);
        ray_t.set_max(t);
        true
    }
}