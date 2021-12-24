use crate::color::Color;
use crate::hittable::Hit;
use crate::ray::Ray;
use crate::utils::Range;
use crate::vec3::Vec3;

pub struct ScatterResult<'a> {
    pub material_color: &'a Color,
    pub scattered_ray: Ray<'a>,
}

pub trait Material {
    fn scatter<'b>(&self, hit: &'b Hit) -> ScatterResult<'b>;
}

pub struct Lambertian {
    pub color: &'static Color,
}

impl Material for Lambertian {
    fn scatter<'b>(&self, hit: &'b Hit) -> ScatterResult<'b> {
        let reflection_vector = Vec3::random_from_range(Range::new(-1.0, 1.0));
        let scattered_ray = Ray::new(&hit.hit_point, &hit.normal + reflection_vector);
        ScatterResult {
            material_color: self.color,
            scattered_ray,
        }
    }
}

// let reflection_vector = match REFLECTION_STRATEGY {
//     ReflectionStrategy::RandomInSphere => GeometricSphere::unit().random_point(),
//     ReflectionStrategy::Lambertian => Vec3::random_from_range(Range::new(-1.0, 1.0)),
//     ReflectionStrategy::Hemispherical => {
//         GeometricSphere::unit().random_point_in_hemisphere(&normal)
//     }
// };
// let reflected_ray = Ray::new(&hit_point, normal + reflection_vector);
