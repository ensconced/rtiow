mod camera;
mod color;
mod hittable;
mod hittable_list;
mod pixel;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::Hit;
use hittable_list::HittableList;
use pixel::Pixel;
use rand::random;
use ray::Ray;
use sphere::Sphere;
use utils::*;
use vec3::Vec3;

const MAX_COLOR: u32 = 255;
const MAX_DEPTH: u32 = 20;
const SAMPLES_PER_PIXEL: u32 = 100;

enum DebugStrategy {
    Normals,
    SingleColor,
    NoDebug,
}

const DEBUG_STRATEGY: DebugStrategy = DebugStrategy::SingleColor;
const DISPLAY_PROGRESS: bool = false;

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

    let sphere1_radius = 0.3;
    world.add(Box::new(Sphere {
        radius: sphere1_radius,
        center: Vec3(0.0, 0.5, -1.0),
    }));

    // let sphere2_radius = 0.5;
    // world.add(Box::new(Sphere {
    //     radius: sphere2_radius,
    //     center: Vec3(0.0, -sphere2_radius - sphere1_radius, -1.0),
    // }));

    // TODO - implement Iterator for Camera to more easily iterate over pixels?
    for row in 0..camera.image_height {
        if DISPLAY_PROGRESS {
            display_progress(camera.image_height, row);
        }

        for col in 0..camera.image_width {
            let pixel_color = match DEBUG_STRATEGY {
                DebugStrategy::Normals => {
                    let x_position = col as f64;
                    let y_position = row as f64;
                    let x_level = x_position / camera.image_width as f64;
                    let y_level = 1.0 - (y_position / camera.image_height as f64);
                    let ray = camera.get_ray(x_level, y_level);
                    if let Some(Hit { normal, .. }) = world.hit(&ray, 0.0, 1.0) {
                        color_by_normal(normal)
                    } else {
                        background(camera.viewport_height, ray)
                    }
                }
                DebugStrategy::SingleColor => {
                    let x_position = col as f64;
                    let y_position = row as f64;
                    let x_level = x_position / camera.image_width as f64;
                    let y_level = 1.0 - (y_position / camera.image_height as f64);
                    let ray = camera.get_ray(x_level, y_level);
                    if let Some(Hit { .. }) = world.hit(&ray, 0.0, 1.0) {
                        Color::red()
                    } else {
                        background(camera.viewport_height, ray)
                    }
                }
                DebugStrategy::NoDebug => {
                    let mut pixel = Pixel::new();
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let pixel_x: f64 = random();
                        let pixel_y: f64 = random();
                        let x_position = col as f64 + pixel_x;
                        let y_position = row as f64 + pixel_y;
                        let x_level = x_position / camera.image_width as f64;
                        let y_level = 1.0 - (y_position / camera.image_height as f64);
                        let ray = camera.get_ray(x_level, y_level);
                        pixel.add_color(color_by_diffuse_reflection(
                            camera.viewport_height,
                            ray,
                            &world,
                            MAX_DEPTH,
                        ));
                    }
                    pixel.get_color()
                }
            };
            print!("{}", pixel_color);
        }
    }
    if DISPLAY_PROGRESS {
        display_done();
    }
}

fn background(viewport_height: f64, ray: Ray) -> Color {
    let direction = ray.vector.unit_vector();
    let vectors_y_range = Range::new(-viewport_height / 2.0, viewport_height / 2.0);
    let new_range = Range::new(0.0, 1.0);
    let upwardsness = remap(direction.y(), &vectors_y_range, &new_range);
    Color::from_vec(lerp(upwardsness, Color::white().vec, Color::sky_blue().vec))
}

fn color_by_normal(normal: Vec3) -> Color {
    let normal_component_range = Range::new(-1.0, 1.0);
    let new_range = Range::new(0.0, 1.0);
    let r = remap(normal.0, &normal_component_range, &new_range);
    let g = remap(normal.1, &normal_component_range, &new_range);
    let b = remap(normal.2, &normal_component_range, &new_range);
    Color::new(r, g, b)
}

fn color_by_diffuse_reflection(
    viewport_height: f64,
    ray: Ray,
    world: &HittableList,
    depth: u32,
) -> Color {
    if let Some(Hit {
        normal, hit_point, ..
    }) = world.hit(&ray, 0.0, 1.0)
    {
        if depth <= 0 {
            return Color::black();
        }
        let unit_sphere = Sphere::new(1.0, &hit_point + normal);
        let target = unit_sphere.random_point();
        let reflected_ray_vector = &hit_point - target;
        let reflected_ray = Ray::new(&hit_point, reflected_ray_vector);
        Color::from_vec(
            color_by_diffuse_reflection(viewport_height, reflected_ray, world, depth - 1).vec * 0.5,
        )
    } else {
        background(viewport_height, ray)
    }
}
