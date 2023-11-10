use anyhow::Result;
use glam::DVec3;

use crate::{
    bvh::node::BVHNode,
    camera::Camera,
    hittable::{hittable_list::HittableList, sphere::Sphere},
    load_image,
    material::*,
    texture::{CheckerTexture, NoiseTexture},
    util::{
        color::Color,
        random::{random_f64, random_vec_in_range},
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

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::ZERO;
    camera.vup = DVec3::Y;

    camera.defocus_angle = 0.0;

    camera.render(&world)
}
