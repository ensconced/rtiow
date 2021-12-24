mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod pixel;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::Hit;
use hittable_list::HittableList;
use material::{Lambertian, Material};
use pixel::Pixel;
use rand::random;
use ray::Ray;
use sphere::{GeometricSphere, ObjectSphere};
use utils::*;
use vec3::Vec3;

const MAX_COLOR: u32 = 255;
const MAX_DEPTH: u32 = 50;
const SAMPLES_PER_PIXEL: u32 = 100;
const SHADOW_ACNE_AVOIDANCE_STEP: f64 = 0.001;

const MATERIAL_A: Lambertian = Lambertian {};
const MATERIAL_B: Lambertian = Lambertian {};

enum DebugStrategy {
    Normals,
    SingleColor,
}

enum ReflectionStrategy {
    RandomInSphere,
    Lambertian,
    Hemispherical,
}

const DEBUG_SETTING: Option<DebugStrategy> = None;
const DISPLAY_PROGRESS: bool = true;
const REFLECTION_STRATEGY: ReflectionStrategy = ReflectionStrategy::Hemispherical;

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

fn create_world<'a>() -> HittableList {
    let mut world = HittableList::new();

    let sphere1_radius = 0.5;
    world.add(Box::new(ObjectSphere::new(
        sphere1_radius,
        Vec3(0.0, 0.0, -1.0),
        &MATERIAL_A,
    )));

    let sphere2_radius = 100.0;
    world.add(Box::new(ObjectSphere::new(
        sphere2_radius,
        Vec3(0.0, -sphere2_radius - sphere1_radius, -1.0),
        &MATERIAL_B,
    )));
    world
}

fn main() {
    let camera = Camera::new(1000, 16.0 / 9.0, 2.0, 1.0, Vec3(0.0, 0.0, 0.0));
    let world = create_world();

    println!("P3"); // means this is an RGB color image in ASCII
    println!("{} {}", camera.image_width, camera.image_height);
    println!("{}", MAX_COLOR);

    let debug_color = |debug_setting, col, row| {
        let x_position = col as f64;
        let y_position = row as f64;
        let x_level = x_position / camera.image_width as f64;
        let y_level = 1.0 - (y_position / camera.image_height as f64);
        let ray = camera.get_ray(x_level, y_level);

        if let Some(Hit { normal, .. }) = world.hit(&ray, SHADOW_ACNE_AVOIDANCE_STEP, f64::INFINITY)
        {
            match debug_setting {
                DebugStrategy::Normals => color_by_normal(normal),
                DebugStrategy::SingleColor => Color::red(),
            }
        } else {
            background(camera.viewport_height, ray)
        }
    };

    for row in 0..camera.image_height {
        if DISPLAY_PROGRESS {
            display_progress(camera.image_height, row);
        }

        for col in 0..camera.image_width {
            let pixel_color = if let Some(debug_setting) = DEBUG_SETTING {
                debug_color(debug_setting, col, row)
            } else {
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
        normal,
        hit_point,
        material,
        ..
    }) = world.hit(&ray, SHADOW_ACNE_AVOIDANCE_STEP, f64::INFINITY)
    {
        if depth == 0 {
            return Color::black();
        }
        let reflection_vector = match REFLECTION_STRATEGY {
            ReflectionStrategy::RandomInSphere => GeometricSphere::unit().random_point(),
            ReflectionStrategy::Lambertian => Vec3::random_from_range(Range::new(-1.0, 1.0)),
            ReflectionStrategy::Hemispherical => {
                GeometricSphere::unit().random_point_in_hemisphere(&normal)
            }
        };
        let reflected_ray = Ray::new(&hit_point, normal + reflection_vector);
        Color::from_vec(
            color_by_diffuse_reflection(viewport_height, reflected_ray, world, depth - 1).vec * 0.5,
        )
    } else {
        background(viewport_height, ray)
    }
}
