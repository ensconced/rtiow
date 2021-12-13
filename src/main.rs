use std::io::{self, Write};

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
const MAX_COLOR: f32 = 255.0;
const BLUE_LEVEL: f32 = 0.25;

fn divide(num: u32, denom: u32) -> f32 {
    num as f32 / denom as f32
}

fn restart_line(stream: &mut impl Write) {
    stream.write_all(b"\x1B[2K\r").unwrap(); // clear line and return cursor to start
}

fn display_progress(row: u32, stream: &mut impl Write) {
    let scanlines_remaining = IMAGE_HEIGHT - row;
    restart_line(stream);
    let msg = format!("scanlines remaining: {}", scanlines_remaining);
    stream.write_all(msg.as_bytes()).unwrap();
}

fn display_done(stream: &mut impl Write) {
    restart_line(stream);
    stream.write_all(b"Done!\n").unwrap();
}

fn main() {
    println!("P3"); // means this is an RGB color image in ASCII
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", MAX_COLOR);

    let stderr = &mut io::stderr();
    for row in 0..IMAGE_HEIGHT {
        display_progress(row, stderr);

        for col in 0..IMAGE_WIDTH {
            let red_level = divide(col, IMAGE_WIDTH - 1);
            let green_level = 1.0 - divide(row, IMAGE_HEIGHT - 1);

            let r = (red_level * MAX_COLOR) as u32;
            let g = (green_level * MAX_COLOR) as u32;
            let b = (BLUE_LEVEL * MAX_COLOR) as u32;

            println!("{} {} {}", r, g, b);
        }
    }
    display_done(stderr);
}
