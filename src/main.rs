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
use material::{Dielectric, Hemispherical, Lambertian, Material, Metal, RandomInSphere};
use pixel::Pixel;
use rand::{random, Rng};
use ray::Ray;
use sphere::ObjectSphere;
use std::{mem, rc::Rc};
use utils::*;
use vec3::Vec3;

const MAX_COLOR: u32 = 255;
const MAX_DEPTH: u32 = 50;
const SAMPLES_PER_PIXEL: u32 = 1;
const SHADOW_ACNE_AVOIDANCE_STEP: f64 = 0.001;
const IMAGE_WIDTH: u32 = 10;
const DEBUG_SETTING: Option<DebugStrategy> = None;
const DISPLAY_PROGRESS: bool = true;
const VERBOSE: bool = false;

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

fn main() {
    let mut world = HittableList::new();

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = random();
            let center = Vec3(
                (a as f64) + 0.9 * random::<f64>(),
                0.2,
                (b as f64) + 0.9 * random::<f64>(),
            );
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_obj = if choose_material < 0.8 {
                    // diffuse
                    let sphere_material = Lambertian::new(Vec3::random() * Vec3::random());
                    ObjectSphere::new(0.2, center, Rc::new(sphere_material))
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Vec3::random().remap(Range::new(0.0, 1.0), Range::new(0.5, 1.0));
                    let mut rng = rand::thread_rng();
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    ObjectSphere::new(0.2, center, Rc::new(sphere_material))
                } else {
                    // glass
                    let sphere_material = Dielectric::new(Vec3(1.0, 1.0, 1.0), 1.5);
                    ObjectSphere::new(0.2, center, Rc::new(sphere_material))
                };
                world.add(Box::new(sphere_obj));
            }
        }
    }

    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let view_up = Vec3(0.0, 1.0, 0.0);
    let lens_radius = 0.05;
    let focus_dist = 10.0;

    let camera = Camera::new(
        IMAGE_WIDTH,
        3.0 / 2.0,
        80.0,
        look_from,
        look_at,
        view_up,
        lens_radius,
        focus_dist,
    );

    let ground_material = Lambertian::new(Vec3(0.5, 0.5, 0.5));

    // ground
    world.add(Box::new(ObjectSphere::new(
        1000.0,
        Vec3(0.0, -1000.0, 0.0),
        Rc::new(ground_material),
    )));

    let material_1 = Dielectric::new(Vec3(1.0, 1.0, 1.0), 1.5);

    world.add(Box::new(ObjectSphere::new(
        1.0,
        Vec3(0.0, 1.0, 0.0),
        Rc::new(material_1),
    )));

    let material_2 = Lambertian::new(Vec3(0.4, 0.2, 0.1));

    world.add(Box::new(ObjectSphere::new(
        1.0,
        Vec3(-4.0, 1.0, 0.0),
        Rc::new(material_2),
    )));

    let material_3 = Metal::new(Vec3(0.7, 0.6, 0.5), 0.0);

    world.add(Box::new(ObjectSphere::new(
        1.0,
        Vec3(4.0, 1.0, 0.0),
        Rc::new(material_3),
    )));

    println!("P3"); // means this is an RGB color image in ASCII
    println!("{} {}", camera.image_width, camera.image_height);
    println!("{}", MAX_COLOR);

    let debug_color = |debug_setting, col, row| {
        let x_position = col as f64;
        let y_position = row as f64;
        let x_level = x_position / camera.image_width as f64;
        let y_level = 1.0 - (y_position / camera.image_height as f64);
        let ray = camera.get_ray(x_level, y_level);

        if let Some(Hit { normal, .. }) = world.hit(ray, SHADOW_ACNE_AVOIDANCE_STEP, f64::INFINITY)
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
    mem::drop(world);
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
    if let Some(hit) = world.hit(ray, SHADOW_ACNE_AVOIDANCE_STEP, f64::INFINITY) {
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
