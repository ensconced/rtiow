use crate::color::Color;
use crate::ray::Ray;
use crate::utils::Range;
use crate::vec3::Vec3;

pub struct ScatterResult<'a> {
    pub material_color: Color,
    pub scattered_ray: Ray<'a>,
}

pub trait Material {
    fn scatter<'a>(&self, ray: Ray<'a>) -> ScatterResult<'a>;
}

pub struct Lambertian {
    color: Color,
}

impl Material for Lambertian {
    fn scatter<'a>(&self, ray: Ray<'a>) -> ScatterResult<'a> {
        let reflection_vector = Vec3::random_from_range(Range::new(-1.0, 1.0));
        let scattered_ray = Ray::new(&hit_point, normal + reflection_vector);
        ScatterResult {
            material_color: Color::red(),
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
