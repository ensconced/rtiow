use std::io::{self, Write};
use std::thread::sleep;
use std::time;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
const MAX_COLOR: f32 = 255.0;
const BLUE_LEVEL: f32 = 0.25;

fn divide(num: u32, denom: u32) -> f32 {
    num as f32 / denom as f32
}

fn display_progress(row: u32) {
    let scanlines_remaining = IMAGE_HEIGHT - row;
    let mut stderr = io::stderr();
    if row > 0 {
        stderr
            .write_all(b"\x1B[2K") // clear line and return cursor to start
            .expect("failed to write to stderr");
        stderr
            .write_all(b"\r") // clear line and return cursor to start
            .expect("failed to write to stderr");
    }
    let msg = format!("scanlines remaining: {}", scanlines_remaining);
    stderr
        .write_all(msg.as_bytes())
        .expect("failed to write to stderr");
}

fn main() {
    println!("P3"); // means this is an RGB color image in ASCII
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", MAX_COLOR);

    for row in 0..IMAGE_HEIGHT {
        sleep(time::Duration::from_millis(100));
        display_progress(row);

        for col in 0..IMAGE_WIDTH {
            let red_level = divide(col, IMAGE_WIDTH - 1);
            let green_level = 1.0 - divide(row, IMAGE_HEIGHT - 1);

            let r = (red_level * MAX_COLOR) as u32;
            let g = (green_level * MAX_COLOR) as u32;
            let b = (BLUE_LEVEL * MAX_COLOR) as u32;

            println!("{} {} {}", r, g, b);
        }
    }
    let mut stderr = io::stderr();
    stderr.write_all(b"\n").expect("oh no");
}
