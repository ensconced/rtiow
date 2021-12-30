use crate::color::Color;
use crate::hittable::Hit;
use crate::ray::Ray;
use crate::sphere::GeometricSphere;
use crate::utils::Range;
use crate::vec3::Vec3;
use rand::random;

pub struct ScatterResult {
    pub material_color: Color,
    pub scattered_ray: Ray,
}

pub trait Material {
    fn scatter<'b>(&self, hit: &'b Hit) -> Option<ScatterResult>;
}

pub struct Lambertian {
    pub color: Color,
}

impl Lambertian {
    pub fn new(color_vec: Vec3) -> Self {
        Self {
            color: Color::from_vec(color_vec),
        }
    }
}

pub struct RandomInSphere {
    pub color: Color,
}

pub struct Hemispherical {
    pub color: Color,
}

pub struct Metal {
    pub color: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(color_vec: Vec3, fuzz: f64) -> Self {
        Self {
            color: Color::from_vec(color_vec),
            fuzz,
        }
    }
}

pub struct Dielectric {
    pub color: Color,
    pub refractive_index: f64,
}

impl Dielectric {
    pub fn new(color_vec: Vec3, refractive_index: f64) -> Self {
        Self {
            color: Color::from_vec(color_vec),
            refractive_index,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit: &Hit) -> Option<ScatterResult> {
        let reflection_vector = Vec3::random_from_range(Range::new(-1.0, 1.0));
        Some(ScatterResult {
            material_color: self.color,
            scattered_ray: Ray::new(hit.hit_point, hit.normal + reflection_vector),
        })
    }
}

impl Material for RandomInSphere {
    fn scatter(&self, hit: &Hit) -> Option<ScatterResult> {
        let reflection_vector = GeometricSphere::unit().random_point_in();
        let mut scatter_direction = hit.normal + reflection_vector;
        // catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }
        Some(ScatterResult {
            material_color: self.color,
            scattered_ray: Ray::new(hit.hit_point, scatter_direction),
        })
    }
}

impl Material for Hemispherical {
    fn scatter(&self, hit: &Hit) -> Option<ScatterResult> {
        let reflection_vector = GeometricSphere::unit().random_point_in_hemisphere(hit.normal);
        Some(ScatterResult {
            material_color: self.color,
            scattered_ray: Ray::new(hit.hit_point, hit.normal + reflection_vector),
        })
    }
}

impl Material for Metal {
    fn scatter(&self, hit: &Hit) -> Option<ScatterResult> {
        let reflected_ray_vector = hit.ray.vector.unit_vector().reflect(hit.normal);
        let fuzz = GeometricSphere::unit().random_point_in() * self.fuzz;
        let vector = reflected_ray_vector + fuzz;
        // fuzz may have taken the vector below the surface
        if vector.dot(hit.normal) < 0.0 {
            return None;
        }
        Some(ScatterResult {
            material_color: self.color,
            scattered_ray: Ray::new(hit.hit_point, vector),
        })
    }
}

fn refract(incident_vector: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min((-incident_vector).dot(normal), 1.0);
    let r_out_perp = (incident_vector + normal * cos_theta) * etai_over_etat;
    let r_out_parallel = normal * -(f64::abs(1.0 - r_out_perp.length_squared())).sqrt();
    return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, hit: &Hit) -> Option<ScatterResult> {
        let refractive_index_ratio = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = hit.ray.vector.unit_vector();
        let cos_theta = f64::min((-unit_direction).dot(hit.normal), 1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = refractive_index_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refractive_index_ratio) > random() {
                unit_direction.reflect(hit.normal)
            } else {
                refract(unit_direction, hit.normal, refractive_index_ratio)
            };
        Some(ScatterResult {
            material_color: self.color,
            scattered_ray: Ray::new(hit.hit_point, direction),
        })
    }
}
