use crate::sphere;
use crate::vec3;

pub struct Ray<'a> {
    origin: &'a vec3::Vec3,
    pub vector: &'a vec3::Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a vec3::Vec3, vector: &'a vec3::Vec3) -> Self {
        Self { origin, vector }
    }

    pub fn hits_sphere(&self, sphere: &sphere::Sphere) -> bool {
        let center_to_ray_origin = self.origin - &sphere.center;
        // a, b, c as in the quadratic formula
        let a = self.vector.dot(self.vector);
        let b = self.vector.dot(&center_to_ray_origin) * 2.0;
        let c = center_to_ray_origin.dot(&center_to_ray_origin) - sphere.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;
        discriminant > 0.0
    }

    // pub fn at(&self, t: f64) -> vec3::Vec3 {
    //     self.origin + &(self.vector * t)
    // }
}
