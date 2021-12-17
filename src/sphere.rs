use crate::hittable::{Hit, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
}

impl Sphere {
    pub fn normal_at(&self, point: &Vec3) -> Vec3 {
        (point - &self.center).unit_vector()
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let center_to_ray_origin = ray.origin - &self.center;
        // a, b, c as in the quadratic formula
        let a = ray.vector.dot(ray.vector);
        let b = ray.vector.dot(&center_to_ray_origin) * 2.0;
        let c = center_to_ray_origin.dot(&center_to_ray_origin) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        // The quadratic formula gives two possible solutions. We want the
        // nearest root, so first we take the one where you subtract the sqrt of
        // the discriminant, which will give the smaller value for t, i.e.
        // closer to the origin (assuming positive t).
        let sqrt_discrim = discriminant.sqrt();
        let mut root = (-b - sqrt_discrim) / (2.0 * a);
        if root < t_min || root > t_max {
            // Try the other result, where you add the sqrt of the discriminant.
            root = (-b + sqrt_discrim) / (2.0 * a);
            if root < t_min || root > t_max {
                return None;
            }
        }

        let hit_point = ray.at(root);
        Some(Hit::new(self.normal_at(&hit_point), ray, root))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_normal() {
        let sphere = Sphere {
            radius: 1.0,
            center: Vec3(0.0, 0.0, 0.0),
        };
        let point = Vec3(1.0, 0.0, 0.0);
        assert_eq!(sphere.normal_at(&point), Vec3(1.0, 0.0, 0.0));
    }
}
