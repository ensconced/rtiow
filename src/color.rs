use std::fmt;

use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Color {
    vec: Vec3,
}

impl Color {
    pub fn new(red_level: f64, green_level: f64, blue_level: f64) -> Self {
        Color {
            vec: Vec3(red_level, green_level, blue_level),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = (self.vec.0 * 255.999) as u32;
        let g = (self.vec.1 * 255.999) as u32;
        let b = (self.vec.2 * 255.999) as u32;
        write!(f, "{} {} {}\n", r, g, b)
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
