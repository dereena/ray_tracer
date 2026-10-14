use glam::DVec3;

use crate::structs::interval::Interval;
use crate::structs::spheres::Sphere;
use crate::structs::bvh::BoundingBox;
use crate::structs::quad::Quad;
use crate::structs::materials::Material;

pub struct Ray {
    pub origin: DVec3,
    pub dir:DVec3,
    pub time:f64,
}

impl Ray {
    pub fn pos(&self,t:f64) -> DVec3{
        self.origin + self.dir * t
    }
}
#[derive(Clone)]
pub enum Hittable {
    Sphere(Sphere),
    Box(BoundingBox),
    Quad(Quad),
}

impl Hittable {
    pub fn hit(&self,ray:&Ray,t_interval:&mut Interval,hit_info:&mut HitInfo) -> bool {
        match self {
            Self::Sphere(sphere) => { sphere.hit(ray,t_interval,hit_info) }
            Self::Box(bounding_box) => bounding_box.hit(ray,t_interval),
            Self::Quad(quad) => quad.hit(ray,t_interval,hit_info)
        }
    }
    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            Self::Sphere(sphere) => sphere.bounding_box(),
            Self::Box(bounding_box) => *bounding_box,
            Self::Quad(quad) => quad.bounding_box(),
        }
    }
}

#[derive(Default)]
pub struct HitInfo {
    pub position:DVec3,
    pub normal:DVec3,
    pub t:f64,
    pub u:f64,
    pub v:f64,
    pub did_hit:bool,
    pub inside_sphere:bool,
    pub material: Material,
}

impl HitInfo{
    pub fn set_normal(&mut self,ray:&Ray,normal:DVec3) {
        if ray.dir.dot(normal) < 0.0 { // if the dot is negative, the vectors are facing opposite directions and the ray is outside the sphere
            self.normal = normal;
            self.inside_sphere = false
        } else { // if the dot is greater than 0, then the vectors align and the ray is inside the sphere
            self.normal = -normal;
            self.inside_sphere = true;
        }
    }
}
