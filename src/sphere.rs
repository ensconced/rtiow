use crate::hittable::Hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct GeometricSphere {
    pub radius: f64,
    pub center: Vec3,
}

#[derive(Clone)]
pub struct ObjectSphere {
    geometry: GeometricSphere,
    material: Arc<dyn Material + Send + Sync>,
}

impl GeometricSphere {
    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        (point - self.center).unit_vector()
    }

    pub fn random_point_in(&self) -> Vec3 {
        loop {
            let vec = Vec3::random_from_range(&(-self.radius..self.radius));
            if vec.length_squared() < self.radius.powi(2) {
                return self.center + vec;
            }
        }
    }

    pub fn random_point_in_hemisphere(&self, normal: Vec3) -> Vec3 {
        let mut point;
        loop {
            point = self.random_point_in();
            if point.dot(normal) > 0.0 {
                break;
            }
        }
        point
    }

    pub fn unit() -> Self {
        Self {
            radius: 1.0,
            center: Vec3(0.0, 0.0, 0.0),
        }
    }
}

impl ObjectSphere {
    pub fn new(radius: f64, center: Vec3, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            geometry: GeometricSphere { radius, center },
            material: material,
        }
    }
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let center_to_ray_origin = ray.origin - self.geometry.center;
        // a, b, c as in the quadratic formula
        let a = ray.vector.dot(ray.vector);
        let b = ray.vector.dot(center_to_ray_origin) * 2.0;
        let c = center_to_ray_origin.dot(center_to_ray_origin) - self.geometry.radius.powi(2);
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
        Some(Hit::new(
            self.geometry.normal_at(hit_point),
            hit_point,
            ray,
            root,
            Arc::clone(&self.material),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_normal() {
        let sphere = GeometricSphere {
            radius: 1.0,
            center: Vec3(0.0, 0.0, 0.0),
        };
        let point = Vec3(1.0, 0.0, 0.0);
        assert_eq!(sphere.normal_at(point), Vec3(1.0, 0.0, 0.0));
    }
}
