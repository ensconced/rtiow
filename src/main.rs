mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::Hit;
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

// position of the eye
// const origin: Vec3 = Vec3(0.0, 0.0, 0.0);

const MAX_COLOR: u32 = 255;

fn divide(num: u32, denom: u32) -> f64 {
    num as f64 / denom as f64
}

fn restart_line() {
    eprint!("\x1B[2K\r"); // clear line and return cursor to start
}

fn display_progress(image_height: u32, row: u32) {
    let scanlines_remaining = image_height - row;
    restart_line();
    eprint!("scanlines remaining: {}", scanlines_remaining);
}

fn display_done() {
    restart_line();
    eprintln!("Done");
}

fn main() {
    let camera = Camera::new(400, 16.0 / 9.0, 2.0, 1.0, Vec3(0.0, 0.0, 0.0));

    println!("P3"); // means this is an RGB color image in ASCII
    println!("{} {}", camera.image_width, camera.image_height);
    println!("{}", MAX_COLOR);

    let mut world = HittableList::new();

    let sphere1_radius = 0.5;
    world.add(Box::new(Sphere {
        radius: sphere1_radius,
        center: Vec3(0.0, 0.0, -1.0),
    }));

    let sphere2_radius = 100.0;
    world.add(Box::new(Sphere {
        radius: sphere2_radius,
        center: Vec3(0.0, -sphere2_radius - sphere1_radius, -1.0),
    }));

    for row in 0..camera.image_height {
        display_progress(camera.image_height, row);

        for col in 0..camera.image_width {
            let x_level = divide(col, camera.image_width - 1);
            let y_level = 1.0 - divide(row, camera.image_height - 1);
            let ray = camera.get_ray(x_level, y_level);
            print!("{}", ray_color(camera.viewport_height, ray, &world));
        }
    }
    display_done();
}

struct Range {
    min: f64,
    max: f64,
}

impl Range {
    fn width(&self) -> f64 {
        self.max - self.min
    }
}

// re-maps a number from one range to another
fn remap(value: f64, original_range: &Range, new_range: &Range) -> f64 {
    let original_width = original_range.width();
    if original_width == 0.0 {
        return new_range.min;
    }
    let level = (value - original_range.min) / original_width;
    new_range.min + level * new_range.width()
}

fn lerp(value: f64, start_value: Vec3, end_value: Vec3) -> Vec3 {
    start_value * (1.0 - value) + end_value * value
}

fn background(viewport_height: f64, ray: Ray) -> Color {
    let direction = ray.vector.unit_vector();
    let vectors_y_range = Range {
        min: -viewport_height / 2.0,
        max: viewport_height / 2.0,
    };
    let new_range = Range { min: 0.0, max: 1.0 };
    let upwardsness = remap(direction.y(), &vectors_y_range, &new_range);
    let sky_blue = Color::new(0.5, 0.7, 1.0);
    let white = Color::new(1.0, 1.0, 1.0);
    Color::from_vec(lerp(upwardsness, white.vec, sky_blue.vec))
}

fn ray_color(viewport_height: f64, ray: Ray, world: &HittableList) -> Color {
    if let Some(Hit { normal, .. }) = world.hit(&ray, 0.0, 1.0) {
        let normal_component_range = Range {
            min: -1.0,
            max: 1.0,
        };
        let new_range = Range { min: 0.0, max: 1.0 };
        let r = remap(normal.0, &normal_component_range, &new_range);
        let g = remap(normal.1, &normal_component_range, &new_range);
        let b = remap(normal.2, &normal_component_range, &new_range);
        Color::new(r, g, b)
    } else {
        background(viewport_height, ray)
    }
}
