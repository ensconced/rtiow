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
use color::{Color, RenderColor};
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use pixel::Pixel;
use rand::{prelude::SliceRandom, random, thread_rng, Rng};
use ray::Ray;
use sphere::ObjectSphere;
use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread,
    thread::JoinHandle,
};
use utils::*;
use vec3::Vec3;

const MAX_COLOR: u32 = 255;
const MAX_DEPTH: u32 = 50;
const SAMPLES_PER_PIXEL: u32 = 50;
const SHADOW_ACNE_AVOIDANCE_STEP: f64 = 0.001;
const IMAGE_WIDTH: u32 = 1000;
const DISPLAY_PROGRESS: bool = true;
const VERBOSE: bool = false;

fn clear_line() {
    eprint!("\x1B[2K");
}

fn move_to_line_start() {
    eprint!("\r");
}

fn move_cursor_up() {
    eprint!("\x1B[A");
}

fn clear_lines(line_count: u32) {
    for _ in 0..line_count {
        move_cursor_up();
        clear_line();
    }
}

fn display_threads_progress(
    progress_receiver: Receiver<ThreadProgress>,
    thread_infos: &[ThreadInfo],
) {
    let mut report = Vec::new();
    for thread_info in thread_infos {
        report.push(ThreadProgress {
            scanlines_remaining: thread_info.row_count,
            thread_idx: thread_info.thread_idx,
        });
    }

    let mut first_time = true;
    while let Ok(thread_progress) = progress_receiver.recv() {
        report[thread_progress.thread_idx as usize] = thread_progress;
        if first_time {
            first_time = false;
        } else {
            clear_lines(thread_infos.len() as u32);
        }
        for thread_info in thread_infos {
            let ThreadProgress {
                scanlines_remaining,
                thread_idx,
                ..
            } = report[thread_info.thread_idx as usize];
            eprintln!(
                "thread {} - scanlines remaining: {}/{}",
                thread_idx, scanlines_remaining, thread_info.row_count
            );
        }
    }
}

fn display_done() {
    clear_line();
    move_to_line_start();
    eprintln!("Done");
}

struct ThreadRowResult {
    row_idx: u32,
    pixels: Vec<RenderColor>,
}

struct ThreadResult {
    rows: Vec<ThreadRowResult>,
}

#[derive(Clone, Copy)]
struct ThreadProgress {
    scanlines_remaining: u32,
    thread_idx: u32,
}

#[derive(Debug)]
struct ThreadInfo {
    row_count: u32,
    thread_idx: u32,
}

fn run_thread(
    thread_idx: u32,
    rows: Vec<u32>,
    camera: Camera,
    world: HittableList,
    result_sender: Sender<ThreadResult>,
    progress_sender: Sender<ThreadProgress>,
) -> JoinHandle<()> {
    let rows_len = (&rows).len();
    thread::spawn(move || {
        let mut thread_result = ThreadResult { rows: Vec::new() };
        if DISPLAY_PROGRESS {
            progress_sender
                .send(ThreadProgress {
                    scanlines_remaining: rows_len as u32,
                    thread_idx,
                })
                .unwrap();
        }
        for (row_idx, row) in rows.iter().enumerate() {
            let mut thread_row_result = ThreadRowResult {
                row_idx: *row,
                pixels: Vec::new(),
            };
            for col in 0..camera.image_width {
                if VERBOSE {
                    eprintln!("ROW {} COL {}", row, col);
                }
                let pixel_color = {
                    let mut pixel = Pixel::new();

                    for i in 0..SAMPLES_PER_PIXEL {
                        let pixel_x: f64 = random();
                        let pixel_y: f64 = random();
                        let x_position = col as f64 + pixel_x;
                        let y_position = *row as f64 + pixel_y;
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
                thread_row_result
                    .pixels
                    .push(RenderColor::from_color(pixel_color));
            }
            thread_result.rows.push(thread_row_result);
            if DISPLAY_PROGRESS {
                progress_sender
                    .send(ThreadProgress {
                        scanlines_remaining: rows.len() as u32 - row_idx as u32 - 1,
                        thread_idx,
                    })
                    .unwrap();
            }
        }
        result_sender.send(thread_result).unwrap();
    })
}

fn get_threads_info(image_height: u32) -> Vec<ThreadInfo> {
    let thread_count = num_cpus::get();
    let rows_per_thread = (image_height as f64 / thread_count as f64).ceil() as usize;
    let rows: Vec<u32> = (0..image_height).collect();
    let mut thread_infos = vec![];

    for (thread_idx, thread_rows) in rows.chunks(rows_per_thread).enumerate() {
        thread_infos.push(ThreadInfo {
            thread_idx: thread_idx as u32,
            row_count: thread_rows.len() as u32,
        });
    }
    thread_infos
}

fn start_threads(
    thread_infos: &[ThreadInfo],
    camera: Camera,
    world: HittableList,
    result_sender: Sender<ThreadResult>,
    progress_sender: Sender<ThreadProgress>,
) {
    let rows_per_thread = (camera.image_height as f64 / thread_infos.len() as f64).ceil() as usize;
    let mut rows: Vec<u32> = (0..camera.image_height).collect();
    let mut rng = thread_rng();
    rows.shuffle(&mut rng);
    let mut thread_rows = rows.chunks(rows_per_thread);

    for thread_info in thread_infos {
        if let Some(rows) = thread_rows.next() {
            let mut vec = Vec::new();
            vec.extend_from_slice(rows);
            run_thread(
                thread_info.thread_idx,
                vec,
                camera,
                world.clone(),
                result_sender.clone(),
                progress_sender.clone(),
            );
        } else {
            panic!("no rows :(");
        }
    }
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
                    ObjectSphere::new(0.2, center, Arc::new(sphere_material))
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Vec3::random().remap(&(0.0..1.0), &(0.5..1.0));
                    let mut rng = rand::thread_rng();
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    ObjectSphere::new(0.2, center, Arc::new(sphere_material))
                } else {
                    // glass
                    let sphere_material = Dielectric::new(Vec3(1.0, 1.0, 1.0), 1.5);
                    ObjectSphere::new(0.2, center, Arc::new(sphere_material))
                };
                world.add(sphere_obj);
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
        20.0,
        look_from,
        look_at,
        view_up,
        lens_radius,
        focus_dist,
    );

    let ground_material = Lambertian::new(Vec3(0.5, 0.5, 0.5));

    // ground
    world.add(ObjectSphere::new(
        1000.0,
        Vec3(0.0, -1000.0, 0.0),
        Arc::new(ground_material),
    ));

    let material_1 = Dielectric::new(Vec3(1.0, 1.0, 1.0), 1.5);

    world.add(ObjectSphere::new(
        1.0,
        Vec3(0.0, 1.0, 0.0),
        Arc::new(material_1),
    ));

    let material_2 = Lambertian::new(Vec3(0.4, 0.2, 0.1));

    world.add(ObjectSphere::new(
        1.0,
        Vec3(-4.0, 1.0, 0.0),
        Arc::new(material_2),
    ));

    let material_3 = Metal::new(Vec3(0.7, 0.6, 0.5), 0.0);

    world.add(ObjectSphere::new(
        1.0,
        Vec3(4.0, 1.0, 0.0),
        Arc::new(material_3),
    ));

    println!("P3"); // means this is an RGB color image in ASCII
    println!("{} {}", camera.image_width, camera.image_height);
    println!("{}", MAX_COLOR);

    let thread_infos = get_threads_info(camera.image_height);
    let (result_sender, result_receiver) = channel::<ThreadResult>();
    let (progress_sender, progress_receiver) = channel::<ThreadProgress>();
    start_threads(&thread_infos, camera, world, result_sender, progress_sender);
    display_threads_progress(progress_receiver, &thread_infos);

    let mut thread_results = vec![];
    for _ in 0..thread_infos.len() {
        thread_results.push(result_receiver.recv().unwrap());
    }

    let mut all_row_results = vec![];
    for thread_result in thread_results {
        for row_result in thread_result.rows {
            all_row_results.push(row_result);
        }
    }
    all_row_results.sort_by_key(|row_result| row_result.row_idx);
    for row_result in all_row_results {
        for pixel in row_result.pixels {
            println!("{}", pixel);
        }
    }

    if DISPLAY_PROGRESS {
        display_done();
    }
}

fn background(ray: Ray) -> Color {
    let direction = ray.vector.unit_vector();
    let upwardsness = remap(direction.y(), &(-1.0..1.0), &(0.0..1.0));
    if VERBOSE {
        eprintln!("direction of {} ...", ray.vector);
        eprintln!("is... {}", direction);
        eprintln!("and upwardsness is {}", upwardsness);
    }
    Color::from_vec(lerp(upwardsness, Color::white().vec, Color::sky_blue().vec))
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
            eprintln!("background: {:?}", bg);
        }
        bg
    }
}
