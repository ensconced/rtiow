use crate::vec3;

pub struct Sphere {
    pub radius: f64,
    pub center: vec3::Vec3,
}

impl Sphere {
    pub fn normal(&self, point: vec3::Vec3) -> vec3::Vec3 {
        (point - &self.center).unit_vector()
    }
}
