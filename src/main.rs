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
use material::{Dielectric, Hemispherical, Lambertian, Metal, RandomInSphere};
use pixel::Pixel;
use rand::random;
use ray::Ray;
use sphere::ObjectSphere;
use utils::*;
use vec3::Vec3;

const MAX_COLOR: u32 = 255;
const MAX_DEPTH: u32 = 50;
const SAMPLES_PER_PIXEL: u32 = 100;
const SHADOW_ACNE_AVOIDANCE_STEP: f64 = 0.001;
const IMAGE_WIDTH: u32 = 1000;
const DEBUG_SETTING: Option<DebugStrategy> = None;
const DISPLAY_PROGRESS: bool = true;
const VERBOSE: bool = false;

const GROUND_MATERIAL: Lambertian = Lambertian {
    color: &Color {
        vec: Vec3(0.8, 0.8, 0.0),
    },
};
const CENTER_MATERIAL: Lambertian = Lambertian {
    color: &Color {
        vec: Vec3(0.1, 0.2, 0.5),
    },
};
const LEFT_MATERIAL: Dielectric = Dielectric {
    color: &Color {
        vec: Vec3(1.0, 1.0, 1.0),
    },
    refractive_index: 1.5,
};
const RIGHT_MATERIAL: Metal = Metal {
    color: &Color {
        vec: Vec3(0.8, 0.6, 0.2),
    },
    fuzz: 0.0,
};

enum DebugStrategy {
    Normals,
    SingleColor,
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

fn create_world<'a>() -> HittableList {
    let mut world = HittableList::new();

    let ball_radius = 0.5;
    let ground_radius = 100.0;

    // ground
    world.add(Box::new(ObjectSphere::new(
        ground_radius,
        Vec3(0.0, -ground_radius - ball_radius, -1.0),
        &GROUND_MATERIAL,
    )));

    world.add(Box::new(ObjectSphere::new(
        ball_radius,
        Vec3(0.0, 0.0, -1.0),
        &CENTER_MATERIAL,
    )));

    world.add(Box::new(ObjectSphere::new(
        0.5,
        Vec3(-1.0, 0.0, -1.0),
        &LEFT_MATERIAL,
    )));

    world.add(Box::new(ObjectSphere::new(
        ball_radius,
        Vec3(1.0, 0.0, -1.0),
        &RIGHT_MATERIAL,
    )));

    world
}

fn main() {
    let look_from = Vec3(3.0, 3.0, 2.0);
    let look_at = Vec3(0.0, 0.0, -1.0);
    let view_up = Vec3(0.0, 1.0, 0.0);
    let lens_radius = 1.0;
    let focus_dist = (look_from - look_at).length();

    let camera = Camera::new(
        IMAGE_WIDTH,
        16.0 / 9.0,
        20.0,
        look_from,
        look_at,
        view_up,
        lens_radius,
        focus_dist,
    );
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
            background(ray)
        }
    };

    for row in 0..camera.image_height {
        if DISPLAY_PROGRESS {
            display_progress(camera.image_height, row);
        }

        for col in 0..camera.image_width {
            if VERBOSE {
                eprintln!("ROW {} COL {}", row, col);
            }
            let pixel_color = if let Some(debug_setting) = DEBUG_SETTING {
                debug_color(debug_setting, col, row)
            } else {
                let mut pixel = Pixel::new();

                for i in 0..SAMPLES_PER_PIXEL {
                    let pixel_x: f64 = random();
                    let pixel_y: f64 = random();
                    let x_position = col as f64 + pixel_x;
                    let y_position = row as f64 + pixel_y;
                    let x_level = x_position / camera.image_width as f64;
                    let y_level = 1.0 - (y_position / camera.image_height as f64);
                    if VERBOSE {
                        eprintln!("SAMPLE {}, x {}, y {}", i, x_level, y_level);
                    }
                    let ray = camera.get_ray(x_level, y_level);
                    pixel.add_color(color_ray(camera.viewport_height, ray, &world, MAX_DEPTH));
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

fn background(ray: Ray) -> Color {
    let direction = ray.vector.unit_vector();
    let upwardsness = remap(direction.y(), Range::new(-1.0, 1.0), Range::new(0.0, 1.0));
    if VERBOSE {
        eprintln!("direction of {} ...", ray.vector);
        eprintln!("is... {}", direction);
        eprintln!("and upwardsness is {}", upwardsness);
    }
    Color::from_vec(lerp(upwardsness, Color::white().vec, Color::sky_blue().vec))
}

fn color_by_normal(normal: Vec3) -> Color {
    let normal_component_range = Range::new(-1.0, 1.0);
    let new_range = Range::new(0.0, 1.0);
    let r = remap(normal.0, normal_component_range, new_range);
    let g = remap(normal.1, normal_component_range, new_range);
    let b = remap(normal.2, normal_component_range, new_range);
    Color::new(r, g, b)
}

fn color_ray(viewport_height: f64, ray: Ray, world: &HittableList, depth: u32) -> Color {
    if VERBOSE {
        eprintln!("coloring ray {:?}", ray);
        eprintln!("depth {}", depth);
    }
    if let Some(hit) = world.hit(&ray, SHADOW_ACNE_AVOIDANCE_STEP, f64::INFINITY) {
        if depth == 0 {
            if VERBOSE {
                eprintln!("hit depth limit: black");
            }
            return Color::black();
        }
        if let Some(scatter_result) = hit.material.scatter(&hit) {
            let scattered_ray = scatter_result.scattered_ray;
            let scattered_ray_color = color_ray(viewport_height, scattered_ray, world, depth - 1);
            if VERBOSE {
                eprintln!("scattered");
            }
            Color::from_vec(scattered_ray_color.vec * scatter_result.material_color.vec)
        } else {
            if VERBOSE {
                eprintln!("black");
            }
            Color::black()
        }
    } else {
        let bg = background(ray);
        if VERBOSE {
            eprintln!("background: {}", bg);
        }
        bg
    }
}
