use crate::color::Color;
use crate::hittable::Hit;
use crate::ray::Ray;
use crate::sphere::GeometricSphere;
use crate::utils::Range;
use crate::vec3::Vec3;

pub struct ScatterResult<'a> {
    pub material_color: &'a Color,
    pub scattered_ray: Ray,
}

pub trait Material {
    fn scatter<'b>(&self, hit: &'b Hit) -> ScatterResult<'b>;
}

pub struct Lambertian {
    pub color: &'static Color,
}

pub struct RandomInSphere {
    pub color: &'static Color,
}

pub struct Hemispherical {
    pub color: &'static Color,
}

pub struct Metal {
    pub color: &'static Color,
}

impl Material for Lambertian {
    fn scatter<'b>(&self, hit: &'b Hit) -> ScatterResult<'b> {
        let reflection_vector = Vec3::random_from_range(Range::new(-1.0, 1.0));
        ScatterResult {
            material_color: self.color,
            scattered_ray: Ray::new(hit.hit_point, hit.normal + reflection_vector),
        }
    }
}

impl Material for RandomInSphere {
    fn scatter<'b>(&self, hit: &'b Hit) -> ScatterResult<'b> {
        let reflection_vector = GeometricSphere::unit().random_point();
        let mut scatter_direction = hit.normal + reflection_vector;
        // catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }
        ScatterResult {
            material_color: self.color,
            scattered_ray: Ray::new(hit.hit_point, scatter_direction),
        }
    }
}

impl Material for Hemispherical {
    fn scatter<'b>(&self, hit: &'b Hit) -> ScatterResult<'b> {
        let reflection_vector = GeometricSphere::unit().random_point_in_hemisphere(hit.normal);
        ScatterResult {
            material_color: self.color,
            scattered_ray: Ray::new(hit.hit_point, hit.normal + reflection_vector),
        }
    }
}

impl Material for Metal {
    fn scatter<'b>(&self, hit: &'b Hit) -> ScatterResult<'b> {
        let reflected_ray_vector = hit.ray.vector.reflect(hit.normal);

        ScatterResult {
            material_color: self.color,
            scattered_ray: Ray::new(hit.hit_point, reflected_ray_vector),
        }
    }
}
