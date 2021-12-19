use std::ops;
pub struct Range {
    pub min: f64,
    pub max: f64,
}

impl Range {
    pub fn width(&self) -> f64 {
        self.max - self.min
    }
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
}

// re-maps a number from one range to another
pub fn remap(value: f64, original_range: &Range, new_range: &Range) -> f64 {
    let original_width = original_range.width();
    if original_width == 0.0 {
        return new_range.min;
    }
    let level = (value - original_range.min) / original_width;
    new_range.min + level * new_range.width()
}

pub fn lerp<T: ops::Mul<f64, Output = T> + ops::Add<Output = T>>(
    level: f64,
    start_value: T,
    end_value: T,
) -> T {
    start_value * (1.0 - level) + end_value * level
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
