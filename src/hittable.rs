use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Hit {
    #[allow(dead_code)]
    point: Vec3,
    pub normal: Vec3,
    pub ray_t: f64,
}

impl Hit {
    pub fn new(point: Vec3, outwards_normal: Vec3, ray: &Ray, ray_t: f64) -> Self {
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
