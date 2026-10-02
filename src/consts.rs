// i made a seperate file cuz i kept forgetting where they were

use glam::DVec3;

pub const INFINITY:f64 = f64::MAX;
pub const PI:f64 = std::f64::consts::PI;

pub const SAMPLES_PER_PIXEL:u32 = 50;
pub const MAX_RECURSION_DEPTH:u32 = 50; // bounces

pub const ROTATION_AXES:DVec3 = DVec3::new(1.0,1.0,0.0);
pub const ROTATION_ANGLES:DVec3 = DVec3::new(0.0,0.0,0.0);

pub const IDEAL_ASPECT_RATIO:f64 = 16.0/10.0;
pub const WINDOW_WIDTH:u32 = 1200;
pub const WINDOW_HEIGHT:u32 = (WINDOW_WIDTH as f64 * (1f64/IDEAL_ASPECT_RATIO)) as u32;
pub const ASPECT_RATIO:f64 = WINDOW_WIDTH as f64 / WINDOW_HEIGHT as f64;

pub const SKY_0:DVec3 = DVec3::ZERO;
pub const SKY_1:DVec3 = DVec3::new(0.28, 0.156, 0.72);
pub const SKY_2:DVec3 = DVec3::new(0.8,0.8,0.8);//0.4,0.2,0.85*);
