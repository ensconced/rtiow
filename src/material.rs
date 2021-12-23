use crate::color::Color;
use crate::ray::Ray;

pub struct Material {}

struct ScatterResult<'a> {
    attenuation: Color,
    scattered: Ray<'a>,
}

impl Material {
    fn scatter(ray: Ray) {}
}
