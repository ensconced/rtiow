use crate::utils::Range;
use rand::{thread_rng, Rng};
use std::{fmt, ops};

#[derive(Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

// Hadamard product
impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

// Hadamard product
impl ops::Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

// Scalar product
impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Vec3 {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_from_range(range: Range) -> Self {
        let mut rng = thread_rng();
        let mut rand_in_range = || -> f64 { range.min + rng.gen::<f64>() * range.width() };
        Self(rand_in_range(), rand_in_range(), rand_in_range())
    }

    pub fn length_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_add_two_vectors() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);
        assert_eq!(v1 + v2, Vec3(5.0, 7.0, 9.0));
    }

    #[test]
    fn can_add_assign_a_vector() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);
        v1 += v2;
        assert_eq!(v1, Vec3(5.0, 7.0, 9.0));
    }

    #[test]
    fn can_subtract_two_vectors() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(6.0, 5.0, 4.0);
        assert_eq!(v2 - v1, Vec3(5.0, 3.0, 1.0));
    }

    #[test]
    fn can_negate_a_vector() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        assert_eq!(-v1, Vec3(-1.0, -2.0, -3.0));
    }

    #[test]
    fn can_get_hadamard_product() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);
        assert_eq!(v1 * v2, Vec3(4.0, 10.0, 18.0));
    }

    #[test]
    fn can_get_scalar_product() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        assert_eq!(v1 * 2.0, Vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn can_mul_assign_a_vector() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        v1 *= 2.0;
        assert_eq!(v1, Vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn can_divide_a_vector() {
        let v1 = Vec3(2.0, 4.0, 6.0);
        assert_eq!(v1 / 2.0, Vec3(1.0, 2.0, 3.0));
    }

    #[test]
    fn can_div_assign_a_vector() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        v1 /= 2.0;
        assert_eq!(v1, Vec3(0.5, 1.0, 1.5));
    }

    #[test]
    fn can_get_dot_product() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, -5.0, 6.0);
        assert_eq!(v1.dot(&v2), 12.0);
    }

    #[test]
    fn can_get_cross_product() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);
        assert_eq!(v1.cross(v2), Vec3(-3.0, 6.0, -3.0));
    }

    #[test]
    fn can_get_length_of_a_vector() {
        let v1 = Vec3(2.0, 10.0, 11.0);
        assert_eq!(v1.length(), 15.0);
    }

    #[test]
    fn can_get_length_squared_of_a_vector() {
        let v1 = Vec3(2.0, 10.0, 11.0);
        assert_eq!(v1.length_squared(), 225.0);
    }

    #[test]
    fn can_get_unit_vector() {
        let v1 = Vec3(2.0, 10.0, 11.0);
        assert_eq!(v1.unit_vector(), Vec3(2.0 / 15.0, 10.0 / 15.0, 11.0 / 15.0));
    }

    #[test]
    fn can_display_a_vector() {
        let v1 = Vec3(1.1, 2.2, 3.3);
        let string = format!("{}", v1);
        assert_eq!(string, "1.1 2.2 3.3");
    }
}
