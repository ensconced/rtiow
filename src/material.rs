use crate::color::Color;
use crate::ray::Ray;

struct ScatterResult<'a> {
    attenuation: Color,
    scattered: Ray<'a>,
}

pub trait Material {
    fn scatter(ray: Ray) -> ScatterResult;
}
