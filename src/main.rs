use anyhow::Result;

use ray_tracing::camera::Camera;
use ray_tracing::hittable_list::HittableList;
use ray_tracing::material::{Dielectric, Lambertian, Metal};
use ray_tracing::sphere::Sphere;
use ray_tracing::{random_range, random_ranged_f64s, Color, Point3};

fn main() -> Result<()> {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));

    world.add(Sphere::new(
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
                    let albedo = random_range(0.0, 1.0) * random_range(0.0, 1.0);
                    Lambertian::new(albedo)
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = random_range(0.5, 1.0);
                    let fuzz = random_ranged_f64s::<1>(0.0, 0.5)[0];
                    Metal::new(albedo, fuzz)
                } else {
                    // Glass
                    Dielectric::new(1.5)
                };

                world.add(Sphere::new(center, 0.2, material));
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    // Render
    camera.render(&world)
}
