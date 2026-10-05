use crate::Interval;
use crate::spheres::Sphere;
use glam::DVec3;
use crate::bvh::BoundingBox;

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
}

impl Hittable {
    pub fn hit(&self,ray:&Ray,t_interval:&mut Interval,hit_info:&mut HitInfo) -> bool {
        match self {
            Self::Sphere(sphere) => { sphere.hit(ray,t_interval,hit_info) }
            Self::Box(bounding_box) => bounding_box.hit(ray,t_interval)
        }
    }
    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            Self::Sphere(sphere) => sphere.bounding_box(),
            Self::Box(bounding_box) => *bounding_box,
        }
    }
}

#[derive(Default)]
pub struct HitInfo {
    pub position:DVec3,
    pub normal:DVec3,
    pub t:f64,
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

#[derive(Copy, Clone)]
pub enum Material {
    Matte(Matte),
    Metal(Metal),
    Glass(Glass),
    Light(Light),
}

impl Default for Material {
    fn default() -> Self {
        Material::Matte(Matte {color:DVec3::ZERO})
    }
}

#[derive(Default,Copy, Clone)]
pub struct Matte {
    pub color:DVec3,
}

#[derive(Default,Copy, Clone)]
pub struct Metal {
    pub color:DVec3,
    pub fuzz:f64,
}

#[derive(Default,Copy, Clone)]
pub struct Glass {
    pub color:DVec3,
    pub refractive_index:f64,
    pub fuzz:f64,
}

#[derive(Default,Copy, Clone)]
pub struct Light {
    pub emission_color:DVec3,
    pub emission_strength:f64,
}
/*
#[derive(Copy,Clone)]
pub struct Material {
    pub reflection_color:DVec3,
    pub emmision_color:DVec3,
    pub emmision_strength:f64,
    pub fuzz:f64,
    pub refractive_index:f64,
    pub _material:usize,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            reflection_color:DVec3::ZERO,
            emmision_color:DVec3::ZERO,
            emmision_strength:0.0,
            fuzz:0.0,
            refractive_index:1.5,
            _material:1,
            
        }
    }
}
 */