use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Hit<'a> {
    pub normal: Vec3,
    pub front_face: bool,
    pub hit_point: Vec3,
    pub ray_t: f64,
    pub ray: Ray,
    pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
    pub fn new(
        outwards_normal: Vec3,
        hit_point: Vec3,
        ray: Ray,
        ray_t: f64,
        material: &'a dyn Material,
    ) -> Self {
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
            front_face: ray_is_from_outside,
            ray,
            normal,
            ray_t,
            hit_point,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    fn get_material(&self) -> &dyn Material;
}
