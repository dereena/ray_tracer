use glam::DVec3;
use crate::consts::*;
use crate::random_unit_in_disk;

pub struct Camera {
    pub position:DVec3,
    pub focal_length:f64,
    pub viewport_width:f64,
    pub viewport_height:f64,
    pub defocus_angle:f64,
    pub disk_u:DVec3,
    pub disk_v:DVec3,
    pub starting_pixel:DVec3,
    pub delta_x:DVec3,
    pub delta_y:DVec3,
}

impl Camera {
    pub fn init(position:DVec3,look_at:DVec3,v_up:DVec3,v_fov:f64) -> Self {
        let focal_length:f64 = FOCAL_LENGTH;
        let defocus_angle:f64 = DEFOCUS_ANGLE;
        let w:DVec3 = (position - look_at).normalize();
        let u:DVec3 = v_up.cross(w).normalize();
        let v:DVec3 = w.cross(u);

        let theta:f64 = v_fov * PI / 180.0;
        let h:f64 = (theta/2.0).tan();
        let viewport_height:f64 = 2.0 * h * focal_length;
        let viewport_width:f64 = viewport_height * ASPECT_RATIO;
        let viewport_u:DVec3 = viewport_width * -u;
        let viewport_v:DVec3 = viewport_height * -v;

        let pshift_x:DVec3 = viewport_u / WINDOW_WIDTH as f64;
        let pshift_y:DVec3 = viewport_v / WINDOW_HEIGHT as f64;
        let starting_pixel:DVec3 = position - (focal_length * w) - viewport_u/2.0 - viewport_v/2.0 + 0.5*(pshift_x + pshift_y);

        let defocus_radius:f64 = focal_length * (defocus_angle*PI/360.0).tan();
        let disk_u:DVec3 = defocus_radius * u;
        let disk_v:DVec3 = defocus_radius * v;


        let camera = Camera {
            position,
            viewport_height,
            viewport_width,
            defocus_angle:DEFOCUS_ANGLE,
            focal_length:FOCAL_LENGTH,
            starting_pixel,
            delta_x:pshift_x,
            delta_y:pshift_y,
            disk_u,
            disk_v,

        };
        camera
    }

    pub fn disk_sample(&self) -> DVec3 {
        let rand_in_disk:DVec3 = random_unit_in_disk();
        self.position + (rand_in_disk.x * self.disk_u) + (rand_in_disk.y * self.disk_v)
    }
}