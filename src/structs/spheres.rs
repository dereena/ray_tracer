use glam::DVec3;
use crate::structs::bvh::BoundingBox;
use crate::lerp;
use crate::structs::rays::*;
use crate::structs::materials::*;
use crate::structs::interval::Interval;

#[derive(Clone)]
pub struct Sphere{
    pub center_1: DVec3,
    pub center_2:DVec3,
    pub radius:f64,
    pub material:Material,
}

impl Sphere {
    pub fn static_sphere(static_center:DVec3,radius:f64,material: Material) -> Self {
        Sphere {
            center_1:static_center,
            center_2:static_center,
            radius,
            material,
        }
    }
    pub fn dynamic_sphere(center_1:DVec3,center_2:DVec3,radius:f64,material: Material) -> Self {
        Sphere {
            center_1,
            center_2,
            radius,
            material,
        }
    }
    pub fn bounding_box(&self) -> BoundingBox{
        let vec:DVec3 = DVec3::splat(self.radius);
        if self.center_1 == self.center_2 {
            BoundingBox::create_box(self.center_1 - vec, self.center_1 + vec)
        } else {
            let box_1:BoundingBox = BoundingBox::create_box(self.center_1 - vec, self.center_1 + vec);
            let box_2:BoundingBox = BoundingBox::create_box(self.center_2 - vec, self.center_2 + vec);
            box_1 + box_2
        }
    }
    pub fn center(&self,time:f64) -> DVec3 {
        lerp(self.center_1,self.center_2,time)
    }
    pub fn hit(&self,ray:&Ray,t_interval:&mut Interval,hit_info:&mut HitInfo) -> bool {
        let center:DVec3 = self.center(ray.time);
        let radius:f64 = self.radius;
        let tmin:f64 = t_interval.min;
        let tmax:f64 = t_interval.max;
        // Using "Ray Tracing in one weekend", we can find the derication of a quadratic used to find intersection with a sphere
        // The quadratic returns t, which shows how many unit vector lengths away the intersection points are
        // If the determinant of the quadratic is < 0, there are no intersections and we return none
        // If the determinant = 0, there is one, otherwise, there are two and we filter for the closest.
        // we then return t or None
        let d:DVec3 = ray.dir;
        let cq:DVec3 = center-ray.origin;
        let a:f64 = d.dot(d);
        //let b:f64 = (-2.0*d).dot(cq); Normally b = -2d • (c-q), but we replace h = b/-2 to simply the quadratic and remove factors of two
        let h:f64 = d.dot(cq);
        let c:f64 = cq.dot(cq) - radius*radius;
        let determinant:f64 = h*h - a * c;
        if determinant < 0.0 {
            return false
        }
        let dsqrt:f64 = determinant.sqrt(); // save computation time by computing once for both roots
        let t_near:f64 = (h - dsqrt)/a;
        let t_far:f64  = (h + dsqrt)/a;
        let root:f64;
        if t_near > tmin && t_near < tmax {
            root = t_near;
        } else if t_far > tmin && t_far < tmax {
            root = t_far;
        } else {
            return false;
        }
        let pos:DVec3 = ray.pos(root);
        hit_info.t = root;
        hit_info.position = pos;
        hit_info.material = self.material;
        hit_info.set_normal(ray,(pos-center)/radius);
        t_interval.set_max(hit_info.t);
        hit_info.did_hit = true;
        true

        // we want the smaller t value because we want the closer intersection point
        // however, t2 could be negative if the sphere is behind the camera
        // so we check if it is negative and if se use t1
        // otherwise t2 is always closer
    }
}


