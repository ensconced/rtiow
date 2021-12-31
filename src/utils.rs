use std::ops;
use std::ops::Range;

pub fn range_width(range: &Range<f64>) -> f64 {
    range.end - range.start
}

// re-maps a number from one range to another
pub fn remap(value: f64, original_range: &Range<f64>, new_range: &Range<f64>) -> f64 {
    let original_width = range_width(original_range);
    if original_width == 0.0 {
        return new_range.start;
    }
    let level = (value - original_range.start) / original_width;
    new_range.start + level * range_width(new_range)
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
