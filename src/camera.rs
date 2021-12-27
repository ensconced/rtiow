use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Camera {
    horizontal: Vec3,
    vertical: Vec3,
    image_bottom_left: Vec3,
    origin: Vec3,
    pub image_height: u32,
    pub image_width: u32,
    pub viewport_height: f64,
}

impl Camera {
    pub fn get_ray(&self, x_level: f64, y_level: f64) -> Ray {
        let ray_image_intersection =
            self.image_bottom_left + self.horizontal * x_level + self.vertical * y_level;
        let ray_vector = ray_image_intersection - self.origin;
        Ray::new(self.origin, ray_vector)
    }
    pub fn new(
        image_width: u32,
        image_aspect_ratio: f64,
        vertical_fov_degrees: f64,
        focal_length: f64, // i.e. distance from "eye" to "image" in space
        origin: Vec3,
    ) -> Self {
        let image_height = (image_width as f64 / image_aspect_ratio) as u32;
        let h = (vertical_fov_degrees.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * image_aspect_ratio;
        let horizontal = Vec3(viewport_width, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);
        let origin_to_image_center = Vec3(0.0, 0.0, -focal_length);
        let image_bottom_left = origin + origin_to_image_center - horizontal / 2.0 - vertical / 2.0;
        Self {
            horizontal,
            vertical,
            image_bottom_left,
            origin,
            image_height,
            image_width,
            viewport_height,
        }
    }
}
