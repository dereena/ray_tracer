use glam::DVec3;

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