use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Camera {
    horizontal: Vec3,
    vertical: Vec3,
    image_bottom_left: Vec3,
    origin: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    pub image_height: u32,
    pub image_width: u32,
    pub viewport_height: f64,
}

impl Camera {
    pub fn get_ray(&self, x_level: f64, y_level: f64) -> Ray {
        let offset_vec = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * offset_vec.x() + self.v * offset_vec.y();
        let ray_origin = self.origin + offset;
        let ray_image_intersection =
            self.image_bottom_left + self.horizontal * x_level + self.vertical * y_level;
        let ray_vector = ray_image_intersection - ray_origin;
        Ray::new(ray_origin, ray_vector)
    }
    pub fn new(
        image_width: u32,
        image_aspect_ratio: f64,
        vertical_fov_degrees: f64,
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        lens_radius: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height = (image_width as f64 / image_aspect_ratio) as u32;
        let h = (vertical_fov_degrees.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * image_aspect_ratio;

        // orthonormal basis vectors
        // w points towards the camera
        let w = (look_from - look_at).unit_vector();
        // u is "right" from the camera's perspective
        let u = view_up.cross(w).unit_vector();
        // the given view_up is not necessarily in the right plane - by taking
        // these cross products we effectively project it onto the plane
        // orthogonal to w.
        let v = w.cross(u);
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let image_bottom_left = look_from - w * focus_dist - horizontal / 2.0 - vertical / 2.0;
        Self {
            u,
            v,
            horizontal,
            vertical,
            image_bottom_left,
            origin: look_from,
            image_height,
            image_width,
            viewport_height,
            lens_radius,
        }
    }
}
