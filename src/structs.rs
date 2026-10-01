use crate::consts::INFINITY;


use glam::DVec3;

pub struct Camera {
    pub position:DVec3,
    pub focal_length:f64,
    pub viewport_width:f64,
    pub viewport_height:f64,
}

pub struct Interval {
    pub min:f64,
    pub max:f64,
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min:0.0001,
            max:INFINITY,
        }
    }
}

impl Interval {
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self,t:f64) -> bool {
        if t >= self.min && t <= self.max{
            return true
        }
        false
    }
    pub fn surrounds(&self,t:f64) -> bool {
        if t > self.min && t < self.max{
            return true
        }
        false
    }
    pub fn clamp(&self,t:f64) -> f64 {
        if t < self.min {return self.min};
        if t > self.max {return self.max};
        t
    }
    pub fn set_min(&mut self,t:f64) {
        self.min = t;
    }
    pub fn set_max(&mut self,t:f64) {
        self.max = t;
    }
}


pub struct Ray {
    pub Origin: DVec3,
    pub Dir:DVec3,
}

impl Ray {
    pub fn pos(&self,t:f64) -> DVec3{
        self.Origin+self.Dir*t
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
        if ray.Dir.dot(normal) < 0.0 { // if the dot is negative, the vectors are facing opposite directions and the ray is outside the sphere
            self.normal = normal;
            self.inside_sphere = false
        } else { // if the dot is greater than 0, then the vectors align and the ray is inside the sphere
            self.normal = -normal;
            self.inside_sphere = true;
        }
    }
}

#[derive(Copy,Clone)]
pub struct Material {
    pub reflection_color:DVec3,
    pub emmision_color:DVec3,
    pub emmision_strength:f64,
    pub fuzz:f64,
    pub _material:usize,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            _material:1,
            reflection_color:DVec3::ZERO,
            emmision_color:DVec3::ZERO,
            emmision_strength:0.0,
            fuzz:0.0,
            
        }
    }
}

pub struct Sphere{
    pub center: DVec3,
    pub radius:f64,
    pub material:Material,
}

impl Sphere {
    pub fn sphere_hit(&self,ray:&Ray,t_interval:&mut Interval,hit_info:&mut HitInfo) -> bool {
        let center:DVec3 = self.center;
        let radius:f64 = self.radius;
        let tmin:f64 = t_interval.min;
        let tmax:f64 = t_interval.max;
        // Using "Ray Tracing in one weekend", we can find the derication of a quadratic used to find intersection with a sphere
        // The quadratic returns t, which shows how many unit vector lengths away the intersection points are
        // If the determinant of the quadratic is < 0, there are no intersections and we return none
        // If the determinant = 0, there is one, otherwise, there are two and we filter for the closest.
        // we then return t or None
        let d:DVec3 = ray.Dir;
        let cq:DVec3 = center-ray.Origin;
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