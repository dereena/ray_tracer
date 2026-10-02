use crate::structs::*;

pub const SPHERES:usize = 6;


pub fn objects() -> [Sphere;SPHERES] {
    let material_1 = Material { // floor
        reflection_color: (0.6, 0.6, 0.6).into(),
        _material:2,
        refractive_index:1.5,
        fuzz:0.5,
        ..Default::default()
    };
    let material_2 = Material { // big
        reflection_color: (1.0, 0.2, 0.4).into(),
        _material: 2,
        fuzz:0.0,
        ..Default::default()
    };
    let material_3 = Material { // med small
        reflection_color: (1.0, 1.0, 1.0).into(),
        _material: 2,
        fuzz:0.0,
        ..Default::default()
    };
    let material_4 = Material { // sun
        emmision_color: (1.0, 1.0, 1.0).into(),
        emmision_strength: 2.0,
        ..Default::default()
    };
    let material_5 = Material { // med big glass
        reflection_color: (0.7,0.7, 0.9).into(),
        _material:3,
        refractive_index:1.5,
        ..Default::default()
    };
    let material_6 = Material { // small
        reflection_color: (0.6, 1.0, 0.7).into(),
        _material: 2,
        fuzz:0.0,
        ..Default::default()
    };


    let sphere_1 = Sphere { // big
        center: (0.0, -10000.5, 3.0).into(),
        radius: 10000.0,
        material: material_1,

    };
    let sphere_2 = Sphere { // med
        center: (0.0, 0.0, 3.0).into(),
        radius: 0.5,
        material: material_2,

    };
    let sphere_3 = Sphere { // small
        center:(1.0,-0.1,2.4).into(),
        radius:0.2,
        material:material_3,
    };
    let sphere_4 = Sphere { // light
        center:(-40.0,24.0,-5.0).into(),
        radius:42.0,
        material:material_4,
    };
    let sphere_5 = Sphere { // glass
        center: (-0.4, -0.2, 2.0).into(),
        radius: 0.3,
        material: material_5,

    };
    let sphere_6 = Sphere { // small
        center:(1.0,-0.4,2.4).into(),
        radius:0.1,
        material:material_6,
    };
    let sphere_list:[Sphere;SPHERES] = [sphere_1,sphere_2,sphere_3,sphere_4,sphere_5,sphere_6];
    sphere_list
}