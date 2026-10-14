use glam::DVec3;
use rand::random_range;

pub fn point_in_square() -> DVec3 {
    // creates a random point from (-0.5,-0.5) to (0.5,0.5) to create slight offsets in the ray direction
    // this variety helps create a smoother blend at the edges of objects (anit-aliasing)
    let rand_x:f64 = random_range(-0.5..0.5);
    let rand_y:f64 = random_range(-0.5..0.5);
    DVec3::new(rand_x,rand_y,0.0)
}


pub fn rand_0() -> f64 {
    random_range(0.0..1.0)
}
pub fn rand_1() -> f64 {
    random_range(-1.0..1.0)
}

pub fn random_unit_vector() -> DVec3 {
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

pub fn random_unit_in_disk() -> DVec3 {
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

pub fn gammify(color:DVec3) -> DVec3 {
    color.clamp(DVec3::ZERO,DVec3::ONE - 0.001).sqrt()
}

pub fn lerp(a:DVec3,b:DVec3,t:f64) -> DVec3 {
    (1.0 - t) * a + t * b
}
