#![allow(non_snake_case)]

pub mod structs;
pub mod consts;
pub mod objects;
pub mod bvh;
pub mod spheres;
pub mod interval;
pub mod camera;

use structs::*;
use interval::*;
use consts::*;
use camera::*;
use objects::*;
use spheres::*;
use bvh::*;


use image::{RgbImage, Rgb, ImageBuffer};
use glam::{DVec3,DMat3};
use std::sync::atomic::{AtomicU32, Ordering};
use rand::random_range;
use std::time::Instant;
use rayon::prelude::*; // allows for optimized multithreaded computations for for_eaches and such

fn main() {
    let timer = Instant::now();
    let mut img:ImageBuffer<Rgb<u8>,Vec<u8>> = RgbImage::new(WINDOW_WIDTH,WINDOW_HEIGHT);



    let look_from:DVec3 =(0.0,0.0,0.0).into();
    let look_at:DVec3 = (DVec3::new(0.0,0.0,1.0)).normalize();
    let v_up:DVec3 = (0.0,1.0,0.0).into();
    let v_fov:f64 = 45.0;
    let camera:Camera = Camera::init(look_from,look_at,v_up,v_fov);

    let objects:Vec<Hittable> = objects();
    iterate_through_pixels(&camera,&objects,&mut img);
    let duration = timer.elapsed();
    println!("Time Elapsed: {duration:?}");
}

fn iterate_through_pixels(camera:&Camera,objects:&Vec<Hittable>,img:&mut ImageBuffer<Rgb<u8>,Vec<u8>>) {
    let bytes_per_row:usize = (WINDOW_WIDTH * 3) as usize; // there are three bytes of color per pixel so we multiply by 3
    let row_tracker:AtomicU32 = AtomicU32::new(0); // special uninterruptable cpu operations
    let buffer:&mut [u8] = &mut *img; // all of the rgb data of the image
    buffer.par_chunks_mut(bytes_per_row).enumerate().for_each(|(y,row)| { // splits buffer into rows by splitting at bytes_per_row

        for (x,pixel) in row.chunks_exact_mut(3).enumerate() {// row is stored as R,G,B,R,G,B.., so we split it into chunks of 3
            // and store the chunks as pixel and position as x
            let colors_u8:[u8;3] = compute_pixel(x,y,camera,objects);
            pixel.copy_from_slice(&colors_u8); // stores color data into pixel, which goes back into row which goes back into the buffer which stores into img
        }
        let rows_done = row_tracker.fetch_add(1, Ordering::Relaxed) + 1; // relaxed like um does something to remove restrictions on clock idk man
        println!("Line: {rows_done:4}/{WINDOW_HEIGHT}");
    });
    img.save("prostuff.png").expect("Something went terribly horribly wrong when saving this image");
}

fn compute_pixel(x:usize,y:usize,camera:&Camera,objects:&Vec<Hittable>) -> [u8;3] {
    let mut pixel_color:DVec3 = DVec3::ZERO;
    for _sample in 0..SAMPLES_PER_PIXEL {
        let offset:DVec3 = point_in_square();
        let pixel_center:DVec3 = camera.starting_pixel + (x as f64+ offset.x) * camera.delta_x + (y as f64 + offset.y) * camera.delta_y;

        let origin:DVec3;
        if camera.defocus_angle > 0.0 {
            origin =  camera.disk_sample()
        } else {
            origin = camera.position
        }
        let ray_direction:DVec3 = (pixel_center - origin).normalize();
        
        let time:f64 = rand_0().powf(1.0/MOTION_BIAS);
        let mut ray:Ray = Ray {
            origin:origin,
            dir:ray_direction,
            time:time,
        };
        pixel_color += Color(&mut ray,objects).min(DVec3::splat(3.0)); // the super complicated function that returns a DVec3 from 0 to 1 at each coord
    }
    let averaged_colors:DVec3 = gammify(pixel_color / SAMPLES_PER_PIXEL as f64);
    let colors_u8:[u8;3] = [(averaged_colors.x * 256.0) as u8, (averaged_colors.y * 256.0) as u8, (averaged_colors.z * 256.0) as u8];
    colors_u8
}

fn _get_rotation_matrix(axes:DVec3,angles:DVec3) -> DMat3 {
    let mut x_rotation:DMat3 = DMat3::IDENTITY;
    let mut y_rotation:DMat3 = DMat3::IDENTITY;
    let mut z_rotation:DMat3 = DMat3::IDENTITY;
    if axes.x == 1.0 {
        x_rotation = DMat3::from_axis_angle((1.0,0.0,0.0).into(),angles.x)
    }
    if axes.y == 1.0 {
        y_rotation = DMat3::from_axis_angle((0.0,1.0,0.0).into(),angles.y)
    }
    if axes.z == 1.0 {
        z_rotation = DMat3::from_axis_angle((0.0,0.0,1.0).into(),angles.z)
    }
    x_rotation*y_rotation*z_rotation
}

fn point_in_square() -> DVec3 {
    // creates a random point from (-0.5,-0.5) to (0.5,0.5) to create slight offsets in the ray direction
    // this variety helps create a smoother blend at the edges of objects (anit-aliasing)
    let rand_x:f64 = random_range(-0.5..0.5);
    let rand_y:f64 = random_range(-0.5..0.5);
    DVec3::new(rand_x,rand_y,0.0)
}


fn rand_0() -> f64 {
    random_range(0.0..1.0)
}
fn rand_1() -> f64 {
    random_range(-1.0..1.0)
}

fn random_unit_vector() -> DVec3 {
    loop {
        let p:DVec3 = DVec3::new(
            rand_1(),
            rand_1(),
            rand_1(),
        );
        let len_sq:f64 = p.length_squared();
        if len_sq > 1e-160 && len_sq <= 1.0 {
            return p / len_sq.sqrt();
        }
    }
}

fn random_unit_in_disk() -> DVec3 {
    loop {
        let p:DVec3 = DVec3::new(
            rand_1(),
            rand_1(),
            0.0
        );
        if p.length_squared() < 1.0 {
            return p
        }
    }
}

fn Color(ray:&mut Ray,objects:&Vec<Hittable>) -> DVec3 {
    let mut ray_color: DVec3 = DVec3::ONE;
    let mut incoming_light: DVec3 = DVec3::ZERO;

    for _i in 0..MAX_RECURSION_DEPTH {
        let mut hit_info: HitInfo = HitInfo::default();
        object_collision(ray, &mut hit_info,objects);
        if hit_info.did_hit {
            diffuse(ray, &hit_info);

            let emitted_light: DVec3 = hit_info.material.emmision_color * hit_info.material.emmision_strength;
            incoming_light += ray_color * emitted_light;
            ray_color *= hit_info.material.reflection_color;

        } else {
            let t = 0.5 * (ray.dir.y + 1.0); // brings value from [-1,1] to [0,1]
            incoming_light += ray_color * lerp(SKY_1,SKY_2,t);
            break;
        }
    }
    incoming_light

}

fn gammify(color:DVec3) -> DVec3 {
    color.clamp(DVec3::ZERO,DVec3::ONE - 0.001).sqrt()
}

fn lerp(a:DVec3,b:DVec3,t:f64) -> DVec3 {
    (1.0 - t) * a + t * b
}





fn object_collision(ray:&mut Ray,hit_info:&mut HitInfo,objects:&Vec<Hittable>) { // checks each sphere to see if the ray hits it
    let mut t_interval:Interval = Interval::default(); // used to make sure closer objects cover farther ones
    for object in objects {
        object.hit(ray,&mut t_interval,hit_info);
    }
}


fn diffuse(ray:&mut Ray,hit_info:&HitInfo) {
    let lambertian_dir:DVec3 = lambertian_diffusion(hit_info);
    if hit_info.material._material == 1 { // matte surfaces, lambertian diffusian
        ray.dir = lambertian_dir;
    } else if hit_info.material._material == 2 { // metal surfaces, perfect reflections
        let reflected_dir:DVec3 = reflect(ray.dir,hit_info.normal).normalize();
        let combined:DVec3 = lerp(reflected_dir,lambertian_dir,hit_info.material.fuzz).normalize();
        ray.dir = combined;
    } else if hit_info.material._material == 3 {
        ray.dir = dielectric_diffusion(ray,hit_info);
    } else {
        ray.dir = hit_info.normal;
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

fn dielectric_diffusion(ray:&Ray,hit_info:&HitInfo) -> DVec3 {
    let mut direction:DVec3;
    let r_index:f64;
    if hit_info.inside_sphere {
        r_index = hit_info.material.refractive_index;
    } else {
        r_index = 1.0 / hit_info.material.refractive_index;
    }

    let cos:f64 = (-ray.dir.normalize()).dot(hit_info.normal).min(1.0);
    let sin:f64 = (1.0-cos*cos).sqrt();
    let can_refract:bool = r_index * sin <= 1.0;
    if can_refract && reflectance(cos,r_index) <= rand_0() {
        direction = refract(ray.dir.normalize(), hit_info.normal, r_index,cos);
    } else {
        direction = reflect(ray.dir,hit_info.normal);
    }
    direction += hit_info.material.fuzz * random_unit_vector();
    return direction
}

fn reflect(ray_dir:DVec3,sphere_normal:DVec3) -> DVec3 {
    return ray_dir - 2.0 * ray_dir.dot(sphere_normal) * sphere_normal
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




/*
fn _iterate_through_pixels(camera:&Camera,spheres:&[Sphere;SPHERES],img:&mut ImageBuffer<Rgb<u8>,Vec<u8>>) {
    let mut y_tracker:u32 = 0;
    for (x,y,pixel) in img.enumerate_pixels_mut() {
        if !(y == y_tracker) {
            y_tracker = y;
            println!("Line: {y_tracker:4}/{WINDOW_HEIGHT}");
        }
        let mut pixel_color:DVec3 = DVec3::ZERO;
        for _sample in 0..SAMPLES_PER_PIXEL {
            let offset:DVec3 = point_in_square();
            let pixel_center:DVec3 = camera.starting_pixel + (x as f64+ offset.x) * camera.delta_x + (y as f64 + offset.y) * camera.delta_y;

            let origin:DVec3;
            if camera.defocus_angle > 0.0 {
                origin =  camera.disk_sample()
            } else {
                origin = camera.position
            }
            let ray_direction:DVec3 = (pixel_center - origin).normalize();
            let mut ray:Ray = Ray {
                origin:origin,
                dir:ray_direction,
            };
            pixel_color += Color(&mut ray,&spheres).min(DVec3::splat(3.0)); // the super complicated function that returns a DVec3 from 0 to 1 at each coord
        }
        let averaged_colors:DVec3 = gammify(pixel_color / SAMPLES_PER_PIXEL as f64);
        let colors_u8:[u8;3] = [(averaged_colors.x * 256.0) as u8, (averaged_colors.y * 256.0) as u8, (averaged_colors.z * 256.0) as u8];
        *pixel = Rgb(colors_u8);
    }
    img.save("prostuff.png").expect("Something went terribly horribly wrong when saving this image");
}
*/