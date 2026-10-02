#![allow(non_snake_case)]

pub mod structs;
pub mod consts;
pub mod objects;

use structs::*;
use consts::*;
use objects::*;

use image::{RgbImage, Rgb, ImageBuffer};
use glam::{DVec3,DMat3};
use rand::random_range;
use std::time::Instant;

fn main() {
    let timer = Instant::now();
    let mut img:ImageBuffer<Rgb<u8>,Vec<u8>> = RgbImage::new(WINDOW_WIDTH,WINDOW_HEIGHT);
    let v_height:f64 = 2.0;
    let v_width:f64 = v_height * ASPECT_RATIO;
    let camera = Camera {
        position: DVec3::new(0.0, 0.0, 0.0),
        focal_length: 2.0,
        viewport_height: v_height,
        viewport_width: v_width,
    };
    let spheres:[Sphere;SPHERES] = objects();
    iterate_through_pixels(&camera,&spheres,&mut img);
    let duration = timer.elapsed();
    println!("Time Elapsed: {duration:?}");
}

fn iterate_through_pixels(camera:&Camera,spheres:&[Sphere;SPHERES],img:&mut ImageBuffer<Rgb<u8>,Vec<u8>>) {
    // Now what we want to do is get the centers of each pixel on the viewport.
    // We will be sending the rays through them later (O = camera position)
    /*
    _____________
    | • • • • • |
    | • • O • • |
    | • • • • • |
    _____________
     */
    let mut y_tracker:u32 = 0;
    let pshift_x:f64 = camera.viewport_width / WINDOW_WIDTH as f64;
    let pshift_y:f64 = -camera.viewport_height / WINDOW_HEIGHT as f64; // the y value is negative because we want to go down the screen since we start at the top corner
    let starting_corner:DVec3 = camera.position + DVec3::new(-camera.viewport_width/2.0,camera.viewport_height/2.0,camera.focal_length);

    let rot_matrix:DMat3 = get_rotation_matrix(ROTATION_AXES,ROTATION_ANGLES);

    for (x,y,pixel) in img.enumerate_pixels_mut() {
        if !(y == y_tracker) {
            y_tracker = y;
            println!("Line: {y_tracker:4}/{WINDOW_HEIGHT}");
        }
        let mut pixel_color:DVec3 = DVec3::ZERO;
        for _sample in 0..SAMPLES_PER_PIXEL {
            let offset:DVec3 = point_in_square();
            let pixel_center:DVec3 = starting_corner + DVec3::new((x as f64 + 0.5 + offset.x) * pshift_x, (y as f64 + 0.5 + offset.y) * pshift_y,0.0);
            let ray_direction:DVec3 = (pixel_center - camera.position).normalize();
            let mut ray:Ray = Ray {
                Origin:camera.position,
                Dir:rot_matrix*ray_direction,
            };
            pixel_color += Color(&mut ray,&spheres); // the super complicated function that returns a DVec3 from 0 to 1 at each coord
        }
        let averaged_colors:DVec3 = gammify(pixel_color / SAMPLES_PER_PIXEL as f64);
        let colors_u8:[u8;3] = [(averaged_colors.x * 256.0) as u8, (averaged_colors.y * 256.0) as u8, (averaged_colors.z * 256.0) as u8];
        *pixel = Rgb(colors_u8);
    }
    img.save("prostuff.png").expect("Something went terribly horribly wrong when saving this image");
}

fn get_rotation_matrix(axes:DVec3,angles:DVec3) -> DMat3 {
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

fn Color(ray:&mut Ray,spheres:&[Sphere;SPHERES]) -> DVec3 {
    let mut ray_color: DVec3 = DVec3::ONE;
    let mut incoming_light: DVec3 = DVec3::ZERO;

    for _i in 0..MAX_RECURSION_DEPTH {
        let mut hit_info: HitInfo = HitInfo::default();
        object_collision(ray, &mut hit_info, spheres);
        if hit_info.did_hit {
            diffuse(ray, &hit_info);

            let emitted_light: DVec3 = hit_info.material.emmision_color * hit_info.material.emmision_strength;
            incoming_light += ray_color * emitted_light;
            ray_color *= hit_info.material.reflection_color;

        } else {
            let t = 0.5 * (ray.Dir.y + 1.0); // brings value from [-1,1] to [0,1]
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





fn object_collision(ray:&mut Ray,hit_info:&mut HitInfo,spheres:&[Sphere;SPHERES]) { // checks each sphere to see if the ray hits it
    let mut t_interval:Interval = Interval::default(); // used to make sure closer objects cover farther ones
    for sphere in spheres {
        sphere.sphere_hit(ray,&mut t_interval,hit_info);
    }
}


fn diffuse(ray:&mut Ray,hit_info:&HitInfo) {
    let lambertian_dir:DVec3 = lambertian_diffusion(hit_info);
    if hit_info.material._material == 1 { // matte surfaces, lambertian diffusian
        ray.Dir = lambertian_dir;
    } else if hit_info.material._material == 2 { // metal surfaces, perfect reflections
        let reflected_dir:DVec3 = reflect(ray.Dir,hit_info.normal).normalize();
        let combined:DVec3 = lerp(reflected_dir,lambertian_dir,hit_info.material.fuzz);
        ray.Dir = combined;
    } else if hit_info.material._material == 3 {
        ray.Dir = dielectric_diffusion(ray,hit_info);
    } else {
        ray.Dir = hit_info.normal;
    }
    ray.Origin = hit_info.position;
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
    let direction:DVec3;
    let r_index:f64;
    if hit_info.inside_sphere {
        r_index = hit_info.material.refractive_index;
    } else {
        r_index = 1.0 / hit_info.material.refractive_index;
    }

    let cos:f64 = (-ray.Dir.normalize()).dot(hit_info.normal).min(1.0);
    let sin:f64 = (1.0-cos*cos).sqrt();
    let can_refract:bool = r_index * sin <= 1.0;
    if can_refract {
        direction = refract(ray.Dir.normalize(), hit_info.normal, r_index,cos);
    } else {
        direction = reflect(ray.Dir,hit_info.normal);
    }
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