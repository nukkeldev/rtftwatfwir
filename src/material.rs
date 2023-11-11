use std::sync::Arc;

use crate::{
    hittable::HitRecord, texture::Texture, util::all::*, util::vec::dvec3_near_zero, Color,
};

pub trait MaterialT {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::ZERO
    }
}

#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
}

impl MaterialT for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
            Material::Dielectric(d) => d.scatter(r_in, rec),
            Material::DiffuseLight(dl) => dl.scatter(r_in, rec),
            Material::Isotropic(i) => i.scatter(r_in, rec),
        }
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        match self {
            Material::Lambertian(l) => l.emitted(u, v, p),
            Material::Metal(m) => m.emitted(u, v, p),
            Material::Dielectric(d) => d.emitted(u, v, p),
            Material::DiffuseLight(dl) => dl.emitted(u, v, p),
            Material::Isotropic(i) => i.emitted(u, v, p),
        }
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: impl Texture + 'static) -> Material {
        Material::Lambertian(Self {
            albedo: Arc::new(a),
        })
    }
}

impl MaterialT for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_dir = rec.normal + random_unit_vector();

        if dvec3_near_zero(&scatter_dir) {
            scatter_dir = rec.normal;
        }

        Some((
            Ray::new_with_time(rec.p, scatter_dir, r_in.time),
            self.albedo.sample(rec.u, rec.v, rec.p),
        ))
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Material {
        Material::Metal(Self {
            albedo,
            fuzz: fuzz.min(1.0),
        })
    }
}

impl MaterialT for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(r_in.direction.normalize(), rec.normal);
        let out = (
            Ray::new_with_time(
                rec.p,
                reflected + self.fuzz * random_unit_vector(),
                r_in.time,
            ),
            self.albedo,
        );

        if out.0.direction.dot(rec.normal) > 0.0 {
            Some(out)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Material {
        Material::Dielectric(Self { ir })
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl MaterialT for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = r_in.direction.normalize();
        let cos_theta = (-unit_dir).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if (refraction_ratio * sin_theta > 1.0)
            || (Self::reflectance(cos_theta, refraction_ratio) > rand::random())
        {
            reflect(unit_dir, rec.normal)
        } else {
            refract(unit_dir, rec.normal, refraction_ratio)
        };

        Some((Ray::new_with_time(rec.p, direction, r_in.time), Color::ONE))
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: impl Texture + 'static) -> Material {
        Material::DiffuseLight(Self {
            emit: Arc::new(texture),
        })
    }
}

impl MaterialT for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.emit.sample(u, v, p)
    }
}

#[derive(Clone)]
pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(a: impl Texture + 'static) -> Material {
        Material::Isotropic(Self {
            albedo: Arc::new(a),
        })
    }
}

impl MaterialT for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scattered = Ray::new_with_time(rec.p, random_unit_vector(), r_in.time);
        let attenuation = self.albedo.sample(rec.u, rec.v, rec.p);

        Some((scattered, attenuation))
    }
}
