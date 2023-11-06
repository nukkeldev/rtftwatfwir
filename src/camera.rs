use std::io::Write;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;
use std::{f64::INFINITY, fs::File};

use anyhow::Result;
use glam::DVec3;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::interval::Interval;
use crate::material::MaterialT;
use crate::{hittable::Hittable, ray::Ray, Color, Point3};
use crate::{random_in_unit_disk, write_color};

#[derive(Debug)]
pub struct Camera {
    /// Ratio of the image width over height.
    pub aspect_ratio: f64,
    /// Width of the rendered image in pixels.
    pub image_width: i32,
    /// Height of the rendered image in pixels, calculated from the width and aspect ratio.
    image_height: i32,
    /// Random samples per pixel.
    pub samples_per_pixel: i32,
    /// Maximum number of ray bounces into scene.
    pub max_depth: i32,

    /// Vertical FOV in degrees.
    pub vfov: f64,
    /// Point the camera is looking from.
    pub lookfrom: Point3,
    /// Point the camera is looking at.
    pub lookat: Point3,
    /// Camera-relative "up" direction.
    pub vup: DVec3,

    /// Variation angle of rays through each pixel.
    pub defocus_angle: f64,
    /// Distance from camera lookfrom point to plane of perfect focus.
    pub focus_dist: f64,

    /// Defocus disk horizontal radius.
    defocus_disk_u: DVec3,
    /// Defocus disk vertical radius.
    defocus_disk_v: DVec3,

    /// Camera frame basis vectors.
    u: DVec3,
    v: DVec3,
    w: DVec3,

    /// Kind of a redunaant property, basically the same as lookfrom.
    origin: Point3,
    /// Location of the top-left pixel relative to the camera.
    pixel00_loc: Point3,
    /// Delta between horizontal pixels.
    pixel_delta_u: DVec3,
    /// Delta between vertical pixels.
    pixel_delta_v: DVec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            image_height: Default::default(),
            vfov: 90.0,
            lookfrom: Point3::NEG_Z,
            lookat: Point3::ZERO,
            vup: DVec3::Y,
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            origin: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
            samples_per_pixel: 10,
            max_depth: 10,
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio).max(1.0) as i32;

        self.origin = self.lookfrom;

        // Determine viewport dimensions.
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = (self.image_width as f64 / self.image_height as f64) * viewport_height;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(self.w).normalize();
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u; // Across horizontal
        let viewport_v = viewport_height * -self.v; // Down Vertical

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.origin - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    /// Get a randomly sampled camera ray for the pixel at location (i, j) originating
    /// from the camera defocus disk.
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.origin
        } else {
            self.defocus_disk_sample()
        };
        let ray_time = rand::random::<f64>();

        Ray::new_with_time(ray_origin, pixel_sample - ray_origin, ray_time)
    }

    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.origin + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    /// Returns a random point in the square surrounding a pixel at the origin.
    fn pixel_sample_square(&self) -> DVec3 {
        let px = -0.5 + rand::random::<f64>();
        let py = -0.5 + rand::random::<f64>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::ZERO;
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, INFINITY)) {
            return if let Some((scattered, attenunation)) = rec.material.scatter(r, &rec) {
                attenunation * Self::ray_color(&scattered, world, depth - 1)
            } else {
                Color::ZERO
            };
        }

        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::ONE + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render(&mut self, world: &dyn Hittable) -> Result<()> {
        self.initialize();

        let mut file = File::create("image.ppm")?;

        write!(
            file,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;

        let now = Instant::now();

        let cam = &*self;
        let image_height = cam.image_height;
        let (sender, recv) = channel();

        thread::spawn(move || {
            let mut rows_done = 0.0;
            loop {
                if let Ok(_) = recv.recv() {
                    rows_done += 1.0;
                    println!("{}%", rows_done / image_height as f64 * 100.0);
                } else {
                    break;
                }
            }
        });

        let colors = (0..self.image_height)
            .into_par_iter()
            .flat_map(|j| {
                let row = (0..self.image_width).into_par_iter().map(move |i| {
                    let mut pixel_color = Color::ZERO;
                    for _ in 0..cam.samples_per_pixel {
                        let r = cam.get_ray(i, j);
                        pixel_color += Camera::ray_color(&r, world, cam.max_depth);
                    }

                    pixel_color
                });
                sender.clone().send(()).unwrap();
                row
            })
            .collect::<Vec<_>>();

        for c in colors {
            write_color(&mut file, c, self.samples_per_pixel);
        }

        println!("Completed in {}s.", now.elapsed().as_secs_f32());

        Ok(())
    }
}
