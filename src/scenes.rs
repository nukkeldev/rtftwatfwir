use anyhow::Result;
use glam::DVec3;

use crate::{
    bvh::node::BVHNode,
    camera::Camera,
    hittable::{
        hittable_list::HittableList,
        instance::{Rotation, Translate},
        new_box,
        quad::Quad,
        sphere::Sphere,
    },
    load_image,
    material::*,
    texture::{CheckerTexture, NoiseTexture},
    util::{
        color::Color,
        random::{random_f64, random_vec_in_range},
        vec::AXIS_Y,
        Point3,
    },
};

pub fn random_spheres() -> Result<()> {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(CheckerTexture::new(1.0, Color::ONE, Color::ZERO));

    world.add(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = random_vec_in_range(0.0, 1.0) * random_vec_in_range(0.0, 1.0);
                    Lambertian::new(albedo)
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = random_vec_in_range(0.5, 1.0);
                    let fuzz = random_f64(0.0, 0.5);
                    Metal::new(albedo, fuzz)
                } else {
                    // Glass
                    Dielectric::new(1.5)
                };

                if choose_mat < 0.8 {
                    world.add(Sphere::new_moving(
                        center,
                        center + DVec3::new(0.0, rand::random::<f64>() / 2.0, 0.0),
                        0.2,
                        material,
                    ));
                } else {
                    world.add(Sphere::new_stationary(center, 0.2, material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new_stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new_stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ));

    let world = BVHNode::from_list(&world);

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.70, 0.80, 1.00);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    // Render
    camera.render(&world)
}

pub fn two_spheres() -> Result<()> {
    let mut world = HittableList::new();

    let checker = Lambertian::new(CheckerTexture::new(
        0.8,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Sphere::new_stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checker.clone(),
    ));
    world.add(Sphere::new_stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker,
    ));

    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.70, 0.80, 1.00);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::ZERO;
    camera.vup = DVec3::Y;

    camera.defocus_angle = 0.0;

    camera.render(&world)
}

pub fn earth() -> Result<()> {
    let earth_texture = Lambertian::new(load_image!("assets/textures/earthmap.jpg"));
    let globe = Sphere::new_stationary(Point3::ZERO, 2.0, earth_texture);

    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.70, 0.80, 1.00);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(0.0, 0.0, 12.0);
    camera.lookat = Point3::ZERO;
    camera.vup = DVec3::Y;

    camera.defocus_angle = 0.0;

    camera.render(&HittableList::new_with(globe))
}

pub fn two_perlin_spheres() -> Result<()> {
    let mut world = HittableList::new();

    let noise_texture = Lambertian::new(NoiseTexture::scaled(2.0));
    world.add(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        noise_texture.clone(),
    ));
    world.add(Sphere::new_stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        noise_texture.clone(),
    ));

    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.70, 0.80, 1.00);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::ZERO;
    camera.vup = DVec3::Y;

    camera.defocus_angle = 0.0;

    camera.render(&world)
}

pub fn quads() -> Result<()> {
    let mut world = HittableList::new();

    let left_red = Lambertian::new(Color::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::new(Color::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new(Color::new(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::new(Color::new(0.2, 0.8, 0.8));

    world.add(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        DVec3::Z * -4.0,
        DVec3::Y * 4.0,
        left_red,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        DVec3::X * 4.0,
        DVec3::Y * 4.0,
        back_green,
    ));
    world.add(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        DVec3::Z * 4.0,
        DVec3::Y * 4.0,
        right_blue,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        DVec3::X * 4.0,
        DVec3::Z * 4.0,
        upper_orange,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        DVec3::X * 4.0,
        DVec3::Z * -4.0,
        lower_teal,
    ));

    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.70, 0.80, 1.00);

    camera.vfov = 80.0;
    camera.lookfrom = Point3::new(0.0, 0.0, 9.0);
    camera.lookat = Point3::ZERO;
    camera.vup = DVec3::Y;

    camera.defocus_angle = 0.0;

    camera.render(&world)
}

pub fn simple_light() -> Result<()> {
    let mut world = HittableList::new();

    let noise_texture = Lambertian::new(NoiseTexture::scaled(4.0));
    world.add(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        noise_texture.clone(),
    ));
    world.add(Sphere::new_stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        noise_texture.clone(),
    ));

    let diff_light = DiffuseLight::new(Color::new(8.0, 8.0, 8.0));
    world.add(Sphere::new_stationary(
        Point3::new(0.0, 7.0, 0.0),
        1.0,
        diff_light.clone(),
    ));
    world.add(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        DVec3::X * 2.0,
        DVec3::Y * 2.0,
        diff_light,
    ));

    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(26.0, 3.0, 6.0);
    camera.lookat = Point3::Y * 2.0;
    camera.vup = DVec3::Y;

    camera.defocus_angle = 0.0;

    camera.render(&world)
}

pub fn cornell_box() -> Result<()> {
    let mut world = HittableList::new();

    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        DVec3::Y * 555.0,
        DVec3::Z * 555.0,
        green,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        DVec3::Y * 555.0,
        DVec3::Z * 555.0,
        red,
    ));
    world.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        DVec3::X * -130.0,
        DVec3::Z * -105.0,
        light,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        DVec3::X * 555.0,
        DVec3::Z * 555.0,
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        DVec3::X * -555.0,
        DVec3::Z * -555.0,
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        DVec3::X * 555.0,
        DVec3::Y * 555.0,
        white.clone(),
    ));

    let box_1 = new_box(
        Point3::ZERO,
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box_1 = Rotation::<AXIS_Y>::new(box_1, 15.0);
    let box_1 = Translate::new(box_1, DVec3::new(265.0, 0.0, 295.0));
    world.add(box_1);

    let box_2 = new_box(
        Point3::ZERO,
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box_2 = Rotation::<AXIS_Y>::new(box_2, -18.0);
    let box_2 = Translate::new(box_2, DVec3::new(130.0, 0.0, 65.0));
    world.add(box_2);

    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 200;
    camera.max_depth = 50;
    camera.background = Color::ZERO;

    camera.vfov = 40.0;
    camera.lookfrom = Point3::new(278.0, 278.0, -800.0);
    camera.lookat = Point3::new(278.0, 278.0, 0.0);
    camera.vup = DVec3::Y;

    camera.defocus_angle = 0.0;

    camera.render(&world)
}
