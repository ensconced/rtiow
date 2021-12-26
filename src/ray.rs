use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub vector: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, vector: Vec3) -> Self {
        Self { origin, vector }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.vector * t
    }
}
