mod color;
mod ray;
mod sphere;
mod vec3;

// dimensions of produced image
const IMAGE_WIDTH: u32 = 400;
const IMAGE_ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / IMAGE_ASPECT_RATIO) as u32;

// dimensions of "image" in space
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * IMAGE_ASPECT_RATIO;

// focal length i.e. distance from "eye" to "image" in space
const FOCAL_LENGTH: f64 = 1.0;

const MAX_COLOR: u32 = 255;

fn divide(num: u32, denom: u32) -> f64 {
    num as f64 / denom as f64
}

fn restart_line() {
    eprint!("\x1B[2K\r"); // clear line and return cursor to start
}

fn display_progress(row: u32) {
    let scanlines_remaining = IMAGE_HEIGHT - row;
    restart_line();
    eprint!("scanlines remaining: {}", scanlines_remaining);
}

fn display_done() {
    restart_line();
    eprintln!("Done");
}

fn main() {
    println!("P3"); // means this is an RGB color image in ASCII
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", MAX_COLOR);

    let origin = vec3::Vec3(0.0, 0.0, 0.0);

    let horizontal = vec3::Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = vec3::Vec3(0.0, VIEWPORT_HEIGHT, 0.0);
    let origin_to_image_center = vec3::Vec3(0.0, 0.0, -FOCAL_LENGTH);
    let image_bottom_left = &origin + origin_to_image_center - &horizontal / 2.0 - &vertical / 2.0;

    let sphere = sphere::Sphere {
        radius: 0.5,
        center: vec3::Vec3(0.0, 0.0, -1.0),
    };

    for row in 0..IMAGE_HEIGHT {
        display_progress(row);

        for col in 0..IMAGE_WIDTH {
            let across_level = divide(col, IMAGE_WIDTH - 1);
            let down_level = divide(row, IMAGE_HEIGHT - 1);
            let ray_image_intersection =
                &image_bottom_left + &horizontal * across_level + &vertical * (1.0 - down_level);
            let ray_vector = ray_image_intersection - &origin;
            let ray = ray::Ray::new(&origin, &ray_vector);
            print!("{}", ray_color(ray, &sphere));
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
fn remap(value: f64, original_range: Range, new_range: Range) -> f64 {
    let original_width = original_range.width();
    if original_width == 0.0 {
        return new_range.min;
    }
    let level = (value - original_range.min) / original_width;
    new_range.min + level * new_range.width()
}

fn lerp(value: f64, start_value: vec3::Vec3, end_value: vec3::Vec3) -> vec3::Vec3 {
    start_value * (1.0 - value) + end_value * value
}

fn background(ray: ray::Ray) -> color::Color {
    let direction = ray.vector.unit_vector();
    let vectors_y_range = Range {
        min: -VIEWPORT_HEIGHT / 2.0,
        max: VIEWPORT_HEIGHT / 2.0,
    };
    let new_range = Range { min: 0.0, max: 1.0 };
    let upwardsness = remap(direction.y(), vectors_y_range, new_range);
    let sky_blue = color::Color::new(0.5, 0.7, 1.0);
    let white = color::Color::new(1.0, 1.0, 1.0);
    color::Color::from_vec(lerp(upwardsness, white.vec, sky_blue.vec))
}

fn ray_color(ray: ray::Ray, sphere: &sphere::Sphere) -> color::Color {
    if ray.hits_sphere(sphere) {
        return color::Color::red();
    }
    background(ray)
}
