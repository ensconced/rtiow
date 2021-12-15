use crate::ray;
use crate::vec3;

pub struct Hit {
    #[allow(dead_code)]
    point: vec3::Vec3,
    pub normal: vec3::Vec3,
    #[allow(dead_code)]
    ray_t: f64,
}

impl Hit {
    pub fn new(point: vec3::Vec3, outwards_normal: vec3::Vec3, ray: &ray::Ray, ray_t: f64) -> Self {
        // The normal should always point against the incident ray i.e. inward
        // if the ray is coming from inside, outward if the ray is coming from
        // outside;
        let ray_is_from_outside = outwards_normal.dot(ray.vector) < 0.0;
        let normal = if ray_is_from_outside {
            outwards_normal
        } else {
            -outwards_normal
        };
        Self {
            point,
            normal,
            ray_t,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
