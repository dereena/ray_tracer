#![allow(non_snake_case)]

mod structs;
mod consts;

use structs::*;
use consts::*;

use image::{RgbImage, Rgb, ImageBuffer};
use glam::DVec3;
use std::f64::consts::PI; // #type: ignore

fn main() {
    let mut img:ImageBuffer<Rgb<u8>,Vec<u8>> = RgbImage::new(WINDOW_WIDTH,WINDOW_HEIGHT);
    let v_height:f64 = 2.0;
    let v_width:f64 = v_height * ASPECT_RATIO;
    let camera = Camera {
        position:DVec3::new(0.0,0.0,0.0),
        focal_length: 1.0,
        viewport_height:v_height,
        viewport_width:v_width,
    };

    let sphere_1 = Sphere {
        center:(0.0,-100.5,1.0).into(),
        radius:100.0,
        _material:1,
    };
    let sphere_2 = Sphere {
        center:(0.0,0.0,1.0).into(),
        radius:0.5,
        _material:2,
    };
    let sphere_3 = Sphere {
        center:(0.0,0.0,0.5).into(),
        radius:0.2,
        _material:3,
    };

    let spheres:[Sphere;SPHERES] = [sphere_1,sphere_2,sphere_3];

    // Now what we want to do is get the centers of each pixel on the viewport.
    // We will be sending the rays through them later (O = camera position)
    /*
    _____________
    | • • • • • |
    | • • O • • |
    | • • • • • |
    _____________
     */
    let pshift_x = camera.viewport_width / WINDOW_WIDTH as f64;
    let pshift_y = camera.viewport_height / WINDOW_HEIGHT as f64;
    for (x,y,pixel) in img.enumerate_pixels_mut() {
        let pixel_center:DVec3 = DVec3::new(
            -camera.viewport_width/2.0 + (x as f64 + 0.5) * pshift_x, // positive x is right
            camera.viewport_height/2.0 - (y as f64 + 0.5) * pshift_y, // positive y is up
               camera.focal_length); // positive z is forward

        let ray_direction = pixel_center - camera.position;
        let ray:Ray = Ray {
            Origin:camera.position,
            Dir:ray_direction.normalize(),
        };
        let colors = Color(&ray,&spheres);
        let colors_u8 = [(colors.x * 256.0) as u8, (colors.y * 256.0) as u8, (colors.z * 256.0) as u8];
        *pixel = Rgb(colors_u8);
    }
    img.save("prostuff.png").expect("oops");
}

fn Color(ray:&Ray,spheres:&[Sphere;SPHERES]) -> DVec3 {
    let mut hit_info:HitInfo = HitInfo::default();
    let mut t_interval:Interval = Interval::default();
    let mut intersected:bool = false;
    let mut colors:DVec3 = DVec3::default(); // we need initializations cuz rust estupido :(
    for sphere in spheres {
        let hit:bool = sphere.sphere_hit(ray,&mut t_interval,&mut hit_info);
        if hit {
            colors = 0.5 * (hit_info.normal + 1.0);
            t_interval.set_max(hit_info.t);
            intersected = true;
        }
    }
    if intersected == true {
        return colors
    } else {
        let a = 0.5 * (ray.Dir.y + 1.0); // brings value from [-1,1] to [0,1]
        let lerp: DVec3 = (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
        lerp
    }
}

/*
fn find_normal(ray:&Ray,sphere:&Sphere,t:f64) -> DVec3 {
    let normal_vec:DVec3 = ray.pos(t) - sphere.center;
    normal_vec/sphere.radius
}
*/