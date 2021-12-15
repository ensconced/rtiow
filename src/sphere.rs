use crate::hittable;
use crate::ray;
use crate::vec3;

pub struct Sphere {
    pub radius: f64,
    pub center: vec3::Vec3,
}

impl Sphere {
    pub fn normal_at(&self, point: &vec3::Vec3) -> vec3::Vec3 {
        (point - &self.center).unit_vector()
    }
}

impl hittable::Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::Hit> {
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
        Some(hittable::Hit {
            ray_t: root,
            normal: self.normal_at(&hit_point),
            point: hit_point,
        })
    }
}
