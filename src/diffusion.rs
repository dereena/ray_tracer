use glam::DVec3;
use crate::{lerp, rand_0, random_unit_vector};
use crate::structs::{Glass, HitInfo, Material, Ray};

pub fn diffuse(ray:&mut Ray, hit_info:&HitInfo) {
    let lambertian_dir:DVec3 = lambertian_diffusion(hit_info);
    match hit_info.material {
        Material::Matte(..) => ray.dir = lambertian_diffusion(hit_info),
        Material::Metal(Metal) => {
            let reflected_dir:DVec3 = reflect(ray.dir,hit_info.normal).normalize();
            let combined:DVec3 = lerp(reflected_dir,lambertian_dir,Metal.fuzz).normalize();
            ray.dir = combined;
        }
        Material::Glass(Glass) => ray.dir = dielectric_diffusion(ray,hit_info,Glass),
        _ => ray.dir = hit_info.normal
    }
    ray.origin = hit_info.position;

}

fn lambertian_diffusion(hit_info:&HitInfo) -> DVec3 {
    let normal:DVec3 = hit_info.normal;
    let mut lambertian:DVec3 = normal + random_unit_vector();
    if  lambertian.length() <= 10f64.powi(-8) {
        lambertian = normal
    }
    lambertian.normalize()
}

fn dielectric_diffusion(ray:&Ray,hit_info:&HitInfo,material:Glass) -> DVec3 {
    let mut direction:DVec3;
    let r_index:f64;
    if hit_info.inside_sphere {
        r_index = material.refractive_index;
    } else {
        r_index = 1.0 / material.refractive_index;
    }

    let cos:f64 = (-ray.dir.normalize()).dot(hit_info.normal).min(1.0);
    let sin:f64 = (1.0-cos*cos).sqrt();
    let can_refract:bool = r_index * sin <= 1.0;
    if can_refract && reflectance(cos,r_index) <= rand_0() {
        direction = refract(ray.dir.normalize(), hit_info.normal, r_index,cos);
    } else {
        direction = reflect(ray.dir,hit_info.normal);
    }
    direction += material.fuzz * random_unit_vector();
    direction
}

fn reflect(ray_dir:DVec3,sphere_normal:DVec3) -> DVec3 {
    ray_dir - 2.0 * ray_dir.dot(sphere_normal) * sphere_normal
}

fn refract(ray_dir:DVec3,sphere_normal:DVec3,refraction_ratio:f64,cos:f64) -> DVec3 {
    let incoming:DVec3 = ray_dir.normalize();
    let outgoing_perp:DVec3 = refraction_ratio * (incoming + cos * sphere_normal); // look at article thingy i dont understand :(
    let outgoing_parallel:DVec3 = -(1.0 - outgoing_perp.length_squared()).abs().sqrt() * sphere_normal;
    let outgoing:DVec3 = outgoing_perp + outgoing_parallel;
    outgoing
}

fn reflectance(cosine:f64,r_index:f64) -> f64 { // I have no clue how this works but its like Schlick's thingy or sumfin
    let mut r:f64 = (1.0-r_index)/(1.0+r_index);
    r = r * r;
    r + (1.0-r) * (1.0-cosine).powi(5)
}