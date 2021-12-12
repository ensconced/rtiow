const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
const MAX_COLOR: f32 = 255.0;
const BLUE_LEVEL: f32 = 0.25;

fn divide(num: u32, denom: u32) -> f32 {
    num as f32 / denom as f32
}

fn main() {
    println!("P3"); // means this is an RGB color image in ASCII
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", MAX_COLOR);

    for row in 0..IMAGE_HEIGHT {
        for col in 0..IMAGE_WIDTH {
            let red_level = divide(col, IMAGE_WIDTH - 1);
            let green_level = 1.0 - divide(row, IMAGE_HEIGHT - 1);

            let r = (red_level * MAX_COLOR) as u32;
            let g = (green_level * MAX_COLOR) as u32;
            let b = (BLUE_LEVEL * MAX_COLOR) as u32;

            println!("{} {} {}", r, g, b);
        }
    }
}
