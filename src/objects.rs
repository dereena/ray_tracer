use std::default::Default;
use crate::structs::*;
use crate::spheres::Sphere;


pub fn objects() -> Vec<Hittable> {

    let floor_material = Material::Glass(Glass {
        color: (0.6, 0.6, 0.6).into(),
        refractive_index:1.5,
        fuzz:0.4
    });
    let metal_material = Material::Metal(Metal {
        color: (1.0, 0.2, 0.4).into(),
        fuzz:0.0,
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
        color: (0.7,0.7, 0.9).into(),
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
        metal_material,
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
        0.2,
        air_material,
    );



    let sphere_list:Vec<Sphere> = vec![metal,metal_2,glass,small_sun,air,sun];
    let mut object_list:Vec<Hittable> = Vec::default();
    sphere_list.iter().for_each(|sphere: &Sphere| {
        object_list.push(Hittable::Sphere(sphere.clone()));
    });

    object_list


}

/*
    let floor_material = Material {
        color:(0.5,0.5,0.5).into(),
        _material:2,
        fuzz:0.0,
        
    };
    let floor = Sphere {
        center:(0.0,-1000.5,0.0).into(),
        radius:1000.0,
        material:floor_material,
    };
    let glass_material = Material {
        refractive_index: 1.5,
        color:(0.9,0.9,1.0).into(),
        _material:3,
        
    };
    let air_material = Material {
        refractive_index: 2.0/3.0,
        color:(0.9,0.9,1.0).into(),
        _material:3,
        
    };
    let glass_1 = Sphere {
        radius:0.5,
        center:(0.0,0.0,2.0).into(),
        material:glass_material,
    };
    let glass_2 = Sphere {
        radius:0.5,
        center:(0.0,0.0,3.0).into(),
        material:glass_material,
    };
    let glass_3 = Sphere {
        radius:0.5,
        center:(0.0,0.0,4.0).into(),
        material:glass_material,
    };
    let glass_4 = Sphere {
        radius:0.5,
        center:(0.0,0.0,5.0).into(),
        material:glass_material,
    };
    let glass_5 = Sphere {
        radius:0.5,
        center:(0.0,0.0,6.0).into(),
        material:glass_material,
    };
    let glass_6 = Sphere {
        radius:0.5,
        center:(0.0,0.0,7.0).into(),
        material:glass_material,
    };let glass_7 = Sphere {
        radius:0.5,
        center:(0.0,0.0,8.0).into(),
        material:glass_material,
    };
    let glass_8 = Sphere {
        radius:0.5,
        center:(0.0,0.0,9.0).into(),
        material:glass_material,
    };
    let glass_9 = Sphere {
        radius:0.5,
        center:(0.0,0.0,10.0).into(),
        material:glass_material,
    };
    let air_1 = Sphere {
        radius:0.3,
        center:(0.0,0.0,2.0).into(),
        material:air_material,
    };
    let air_2 = Sphere {
        radius:0.3,
        center:(0.0,0.0,3.0).into(),
        material:air_material,
    };
    let air_3 = Sphere {
        radius:0.3,
        center:(0.0,0.0,4.0).into(),
        material:air_material,
    };
    let air_4 = Sphere {
        radius:0.3,
        center:(0.0,0.0,5.0).into(),
        material:air_material,
    };
    let air_5 = Sphere {
        radius:0.3,
        center:(0.0,0.0,6.0).into(),
        material:air_material,
    };
    let air_6 = Sphere {
        radius:0.3,
        center:(0.0,0.0,7.0).into(),
        material:air_material,
    };let air_7 = Sphere {
        radius:0.3,
        center:(0.0,0.0,8.0).into(),
        material:air_material,
    };
    let air_8 = Sphere {
        radius:0.3,
        center:(0.0,0.0,9.0).into(),
        material:air_material,
    };
    let air_9 = Sphere {
        radius:0.3,
        center:(0.0,0.0,10.0).into(),
        material:air_material,
    };


    let sphere_list:[Sphere;SPHERES] = [floor,glass_1,glass_2,glass_3,glass_4,glass_5,glass_6,glass_7,glass_8,glass_9,air_1,air_2,air_3,air_4,air_5,air_6,air_7,air_8,air_9];
    */