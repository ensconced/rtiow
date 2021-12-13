use std::ops;

#[derive(Debug)]
pub struct Vec3(f64, f64, f64);

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Vec3 {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Vec3 {
    fn length_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
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
    fn can_negate_a_vector() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        assert_eq!(-v1, Vec3(-1.0, -2.0, -3.0));
    }

    #[test]
    fn can_mul_assign_a_vector() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        v1 *= 2.0;
        assert_eq!(v1, Vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn can_div_assign_a_vector() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        v1 /= 2.0;
        assert_eq!(v1, Vec3(0.5, 1.0, 1.5));
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
}
