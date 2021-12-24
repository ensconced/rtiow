use crate::color::Color;
use crate::ray::Ray;

struct ScatterResult<'a> {
    attenuation: Color,
    scattered: Ray<'a>,
}

pub trait Material {
    fn scatter<'a>(&self, ray: Ray<'a>) -> ScatterResult<'a>;
}

pub struct Lambertian {}

impl Material for Lambertian {
    fn scatter<'a>(&self, ray: Ray<'a>) -> ScatterResult<'a> {
        ScatterResult {
            attenuation: Color::red(),
            scattered: ray,
        }
    }
}
