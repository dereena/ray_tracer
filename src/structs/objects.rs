use std::default::Default;
use crate::structs::rays::*;
use crate::structs::spheres::Sphere;
use crate::structs::materials::*;
use crate::structs::quad::Quad;


pub fn objects() -> Vec<Hittable> {
    let orb = Sphere::static_sphere(
        (0.0,0.0,3.5).into(),
        0.5,
        Material::Metal(Metal {
            color:(0.5,0.5,0.5).into(),
            fuzz:0.0,
        })
    );
    let sun = Sphere::static_sphere(
        (0.0,42.0,0.0).into(),
        15.0,
        Material::Light(Light {
            emission_color:(1.0,1.0,1.0).into(),
            emission_strength:5.0,
        }),

    );

    let red_floor = Quad::init(
        (-1.5,-1.0,2.0).into(),
        (0.0,0.0,3.0).into(),
        (3.0,0.0,0.0).into(),
        Material::Metal(Metal {
            color:(0.8,0.1,0.1).into(),
            fuzz:0.1,
        })
    );
    let green_wall = Quad::init(
        (-1.5,-1.0,2.0).into(),
        (0.0,0.0,3.0).into(),
        (0.0,3.0,0.0).into(),
        Material::Light(Light {
            emission_color:(0.2,0.8,0.2).into(),
            emission_strength:4.0,
        })
    );



    let sphere_list:Vec<Sphere> = vec![orb,sun];
    let quad_list:Vec<Quad> = vec![red_floor,green_wall];
    let mut object_list:Vec<Hittable> = Vec::default();
    sphere_list.iter().for_each(|sphere: &Sphere| {
        object_list.push(Hittable::Sphere(sphere.clone()));
    });
    quad_list.iter().for_each(|quad: &Quad| {
        object_list.push(Hittable::Quad(quad.clone()));
    });

    object_list
}
/*
    let floor_material = Material::Glass(Glass {
        color: (0.6, 0.6, 0.6).into(),
        refractive_index:1.5,
        fuzz:0.4
    });
    let metal_material = Material::Metal(Metal {
        color: (1.0, 0.2, 0.4).into(),
        fuzz:0.0,
    });
    let pink_material = Material::Metal(Metal {
        color: (1.0, 0.2, 0.4).into(),
        fuzz:0.2,
    });
    let metal_material_2 = Material::Metal(Metal { // med small
        color: (1.0, 1.0, 1.0).into(),
        fuzz:0.0,
    }) ;
    let sun_material = Material::Light(Light { // sun
        emission_color: (1.0, 1.0, 1.0).into(),
        emission_strength: 2.0,
    });
    let glass_material = Material::Glass(Glass { // med big glass
        color: (1.0,1.0,1.0).into(),
        refractive_index:1.5,
        fuzz:0.0,
    });
    let small_sun = Material::Metal(Metal { // small
        color:(0.6,0.7,0.6).into(),
        fuzz:0.0,
    });
    let air_material = Material::Glass(Glass { // air bubble
        color: (1.0,1.0, 1.0).into(),
        refractive_index:2.0/3.0,
        fuzz:0.0,
    });


    let _floor = Sphere::static_sphere(
        (0.0, -10000.5, 3.0).into(),
        10000.0,
        floor_material
    );
    let metal = Sphere::static_sphere( // med
        (0.0, 0.0, 3.0).into(),
        0.5,
        pink_material,
    );
    let metal_2 = Sphere::static_sphere( // small
        (1.0,-0.1,2.4).into(),
        0.2,
        metal_material_2,
    );
    let sun = Sphere::static_sphere( // light
         (-50.0,26.0,-24.0).into(),
        42.0,
        sun_material,
    );
    let glass = Sphere::static_sphere( // glass
        (-0.2, -0.2, 2.0).into(),
        0.3,
        glass_material,

    );
    let small_sun = Sphere::static_sphere( // small
        (1.0,-0.4,2.4).into(),
        0.1,
        small_sun,
    );
    let air = Sphere::static_sphere( // air bubble
        (-0.2, -0.2, 2.0).into(),
        0.29,
        air_material,
    );


    let wall = Quad::init(
        (-1.0,-1.0,0.0).into(), // starting_pos
        (0.0,10.0,0.0).into(), // side 1
        (0.0,0.0,500.0).into(), // side 2
        Material::Light (Light {
            emission_color: (1.0,1.0,1.0).into(),
            emission_strength: 1.2,
        })
    );
    let floor = Quad::init(
        (-50.0,-0.5,0.0).into(),
        (100.0,0.0,0.0).into(),
        (0.0,0.0,100.0).into(), floor_material);


    let sphere_list:Vec<Sphere> = vec![metal,metal_2,glass,small_sun,air,sun];
    let quad_list:Vec<Quad> = vec![floor,wall];
    let mut object_list:Vec<Hittable> = Vec::default();
    sphere_list.iter().for_each(|sphere: &Sphere| {
        object_list.push(Hittable::Sphere(sphere.clone()));
    });
    quad_list.iter().for_each(|quad: &Quad| {
        object_list.push(Hittable::Quad(quad.clone()));
    });

    object_list
}
*/
