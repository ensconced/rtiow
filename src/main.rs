const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
const MAX_COLOR: u32 = 255;
// const BLUE_LEVEL: f32 = 0.25;

fn main() {
    // P3 means this is an RGB color image in ASCII
    println!("P3");

    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);

    // max color
    println!("{}", MAX_COLOR);

    for row in 0..IMAGE_HEIGHT {
        for col in 0..IMAGE_WIDTH {
            let r = col;
            let g = IMAGE_HEIGHT - 1 - row;
            let b = 63;
            println!("{} {} {}", r, g, b);
        }
    }
}
