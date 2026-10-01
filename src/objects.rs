use crate::consts::SPHERES;
use crate::structs::*;


pub fn objects() -> [Sphere;SPHERES] {
    let material_1 = Material { // big
        reflection_color: (0.5, 0.5, 0.5).into(),
        _material:2,
        fuzz:0.4,
        ..Default::default()
    };
    let material_2 = Material { // med
        reflection_color: (1.0, 0.2, 0.4).into(),
        ..Default::default()
    };
    let material_3 = Material { // small
        reflection_color: (1.0, 1.0, 1.0).into(),
        _material: 2,
        ..Default::default()
    };
    let material_4 = Material { // light
        emmision_color: (0.6, 0.6, 1.0).into(),
        emmision_strength: 4.0,
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
        center:(-0.9,0.05,3.0).into(),
        radius:0.2,
        material:material_3,
    };
    let sphere_4 = Sphere { // light
        center:(-40.0,24.0,-5.0).into(),
        radius:42.0,
        material:material_4,
    };
    let sphere_list:[Sphere;SPHERES] = [sphere_1,sphere_2,sphere_3,sphere_4];
    sphere_list
}