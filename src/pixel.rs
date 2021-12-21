use crate::color::Color;
use crate::vec3::Vec3;

pub struct Pixel {
    cumulative_color: Vec3,
    sample_count: u32,
}

impl Pixel {
    pub fn add_color(&mut self, color: Color) {
        self.sample_count += 1;
        self.cumulative_color.0 += color.r();
        self.cumulative_color.1 += color.g();
        self.cumulative_color.2 += color.b();
    }

    pub fn get_color(&self) -> Color {
        Color::new(
            self.cumulative_color.0 / (self.sample_count as f64),
            self.cumulative_color.1 / (self.sample_count as f64),
            self.cumulative_color.2 / (self.sample_count as f64),
        )
    }

    pub fn new() -> Self {
        Pixel {
            sample_count: 0,
            cumulative_color: Vec3(0.0, 0.0, 0.0),
        }
    }
}
