use std::fmt;

use crate::utils::clamp;

use crate::vec3::Vec3;
#[allow(unused_imports)]
use rand::random;

#[derive(Debug)]
pub struct Color {
    pub vec: Vec3,
}

impl Color {
    pub fn new(red_level: f64, green_level: f64, blue_level: f64) -> Self {
        Color {
            vec: Vec3(red_level, green_level, blue_level),
        }
    }
    pub fn from_vec(vec: Vec3) -> Self {
        Color { vec }
    }

    #[allow(dead_code)]
    pub fn random() -> Self {
        Color {
            vec: Vec3(rand::random(), rand::random(), rand::random()),
        }
    }

    #[allow(dead_code)]
    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn sky_blue() -> Self {
        Self::new(0.5, 0.7, 1.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn r(&self) -> f64 {
        self.vec.0
    }

    pub fn g(&self) -> f64 {
        self.vec.1
    }

    pub fn b(&self) -> f64 {
        self.vec.2
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn gamma_correct(color_component: f64) -> f64 {
            color_component.sqrt()
        }

        fn scale_color(color_component: f64) -> u32 {
            // make sure we don't overshoot from to 255 to 256
            let clamped = clamp(color_component, 0.0, 0.999);
            (gamma_correct(clamped) * 256.0) as u32
        }

        writeln!(
            f,
            "{} {} {}",
            scale_color(self.r()),
            scale_color(self.g()),
            scale_color(self.b())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_display_color() {
        assert_eq!(format!("{}", Color::new(0.0, 0.5, 1.0)), "0 127 255\n");
    }
}
