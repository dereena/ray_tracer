// i made a seperate file cuz i kept forgetting where they were

use glam::DVec3;

pub const INFINITY:f64 = f64::MAX;
pub const PI:f64 = std::f64::consts::PI;

pub const SPHERES:usize = 4;
pub const SAMPLES_PER_PIXEL:u32 = 300;
pub const MAX_RECURSION_DEPTH:u8 = 25; // bounces
pub const DIFFUSION_PROBABILITY:f64 = 1.0; // chance for rays to be absorbed into a lambertian material

pub const ROTATION_AXES:DVec3 = DVec3::new(1.0,1.0,0.0);
pub const ROTATION_ANGLES:DVec3 = DVec3::new(0.0,0.0,0.0);

pub const IDEAL_ASPECT_RATIO:f64 = 16.0/10.0;
pub const WINDOW_WIDTH:u32 = 2140;
pub const WINDOW_HEIGHT:u32 = (WINDOW_WIDTH as f64 * (1f64/IDEAL_ASPECT_RATIO)) as u32;
pub const ASPECT_RATIO:f64 = WINDOW_WIDTH as f64 / WINDOW_HEIGHT as f64;
pub const SKY:DVec3 = DVec3::ZERO;//DVec3::new(0.34, 0.67, 0.9);
pub const SKY_2:DVec3 = DVec3::ZERO;//DVec3::new(0.4,0.2,0.85);