use crate::ray;
use crate::vec3;

pub struct Hit {
    pub point: vec3::Vec3,
    pub normal: vec3::Vec3,
    pub ray_t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
