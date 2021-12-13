// mod vec3;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
const MAX_COLOR: f64 = 255.0;
const BLUE_LEVEL: f64 = 0.25;

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

    for row in 0..IMAGE_HEIGHT {
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
    display_done();
}
