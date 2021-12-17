use crate::vec3::Vec3;

pub struct Ray<'a> {
    pub origin: &'a Vec3,
    pub vector: Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Vec3, vector: Vec3) -> Self {
        Self { origin, vector }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + &self.vector * t
    }
}
