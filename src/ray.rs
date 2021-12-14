use crate::vec3;

pub struct Ray<'a> {
    // origin: &'a vec3::Vec3,
    pub vector: &'a vec3::Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(_origin: &'a vec3::Vec3, vector: &'a vec3::Vec3) -> Self {
        Self {
            // origin,
            vector,
        }
    }

    // pub fn at(&self, t: f64) -> vec3::Vec3 {
    //     self.origin + &(self.vector * t)
    // }
}
