use std::f32::INFINITY;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;

use anyhow::Result;
use glam::Vec3A;
use memmap2::MmapMut;
use rayon::prelude::{IndexedParallelIterator, ParallelIterator};
use rayon::slice::{ParallelSlice, ParallelSliceMut};

use crate::hittable::Hittable;
use crate::material::MaterialT;
use crate::util::all::*;

#[derive(Debug)]
pub struct Camera {
    /// Ratio of the image width over height.
    pub aspect_ratio: f32,
    /// Width of the rendered image in pixels.
    pub image_width: i32,
    /// Height of the rendered image in pixels, calculated from the width and aspect ratio.
    image_height: i32,
    /// Random samples per pixel.
    pub samples_per_pixel: i32,
    /// Maximum number of ray bounces into scene.
    pub max_depth: i32,
    /// Background Color
    pub background: Color,

    /// Vertical FOV in degrees.
    pub vfov: f32,
    /// Point the camera is looking from.
    pub lookfrom: Point3,
    /// Point the camera is looking at.
    pub lookat: Point3,
    /// Camera-relative "up" direction.
    pub vup: Vec3A,

    /// Variation angle of rays through each pixel.
    pub defocus_angle: f32,
    /// Distance from camera lookfrom point to plane of perfect focus.
    pub focus_dist: f32,

    /// Defocus disk horizontal radius.
    defocus_disk_u: Vec3A,
    /// Defocus disk vertical radius.
    defocus_disk_v: Vec3A,

    /// Camera frame basis vectors.
    u: Vec3A,
    v: Vec3A,
    w: Vec3A,

    /// Kind of a redunaant property, basically the same as lookfrom.
    origin: Point3,
    /// Location of the top-left pixel relative to the camera.
    pixel00_loc: Point3,
    /// Delta between horizontal pixels.
    pixel_delta_u: Vec3A,
    /// Delta between vertical pixels.
    pixel_delta_v: Vec3A,
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
            vup: Vec3A::Y,
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
            background: Color::ZERO,
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio).max(1.0) as i32;

        self.origin = self.lookfrom;

        // Determine viewport dimensions.
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = (self.image_width as f32 / self.image_height as f32) * viewport_height;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(self.w).normalize();
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u; // Across horizontal
        let viewport_v = viewport_height * -self.v; // Down Vertical

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

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
            self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.origin
        } else {
            self.defocus_disk_sample()
        };
        let ray_time = rand::random::<f32>();

        Ray::new_with_time(ray_origin, pixel_sample - ray_origin, ray_time)
    }

    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.origin + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    /// Returns a random point in the square surrounding a pixel at the origin.
    fn pixel_sample_square(&self) -> Vec3A {
        let px = -0.5 + rand::random::<f32>();
        let py = -0.5 + rand::random::<f32>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::ZERO;
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, INFINITY)) {
            let color_from_emmision = rec.material.emitted(rec.u, rec.v, rec.p);

            let color_from_scatter =
                if let Some((scattered, attenunation)) = rec.material.scatter(r, &rec) {
                    attenunation * self.ray_color(&scattered, world, depth - 1)
                } else {
                    return color_from_emmision;
                };

            return color_from_emmision + color_from_scatter;
        }

        self.background
    }

    pub fn render(&mut self, world: &dyn Hittable) -> Result<()> {
        self.initialize();

        let total_pixels = self.image_height * self.image_width;
        let pixels_per_percent = total_pixels / 100;
        let file_size = (total_pixels * 3) as u64;

        let mut indices = (0..self.image_height)
            .flat_map(|j| (0..self.image_width).map(move |i| (false, (i, j))))
            .collect::<Vec<_>>();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("_image.raw")?;

        if Path::new("_image.raw").exists() {
            let contents = std::fs::read("_image.raw")?;
            if contents.len() == file_size as usize {
                indices = contents
                    .par_chunks(3)
                    .zip(indices)
                    .map(|(c, (_, p))| (c != &[0, 0, 0], p))
                    .collect();
            } else {
                std::fs::remove_file("_image.raw")?;
                file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open("_image.raw")?;
            }
        }

        file.set_len(file_size)?;
        let mut mmap = unsafe { MmapMut::map_mut(&file)? };

        let pixels = mmap.par_chunks_mut(3).zip(indices);

        let now = Instant::now();
        let (sender, recv) = channel::<()>();

        thread::spawn(move || {
            let mut pixels_done = 0;
            loop {
                if let Ok(_) = recv.recv() {
                    pixels_done += 1;
                    if pixels_done % pixels_per_percent == 0 {
                        println!(
                            "{}%",
                            (pixels_done as f32 / total_pixels as f32 * 100.0) as u8
                        );
                    }
                } else {
                    println!("Rendered in {} seconds!", now.elapsed().as_secs_f32());
                    println!("Writing {total_pixels} pixels...");
                    break;
                }
            }
        });

        pixels.for_each(|(out, (done, (i, j)))| {
            if done {
                sender.send(()).unwrap();
                return;
            }

            let mut pixel_color = Color::ZERO;
            for _ in 0..self.samples_per_pixel {
                let r = self.get_ray(i, j);
                pixel_color += self.ray_color(&r, world, self.max_depth);
            }

            out.copy_from_slice(&convert_color(pixel_color, self.samples_per_pixel));
            sender.send(()).unwrap();
        });
        drop(sender);

        image::save_buffer(
            "image.jpg",
            &mmap,
            self.image_width as u32,
            self.image_height as u32,
            image::ColorType::Rgb8,
        )?;

        drop(mmap);
        std::fs::remove_file("_image.raw")?;

        println!("Completed in {}s.", now.elapsed().as_secs_f32());

        Ok(())
    }
}
