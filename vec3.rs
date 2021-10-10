use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(self) -> f32 {
        self.e[0]
    }

    pub fn y(self) -> f32 {
        self.e[1]
    }

    pub fn z(self) -> f32 {
        self.e[2]
    }

    pub fn r(self) -> f32 {
        self.e[0]
    }

    pub fn g(self) -> f32 {
        self.e[1]
    }

    pub fn b(self) -> f32 {
        self.e[2]
    }

    pub fn length(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f32) -> Self::Output {
        let k: f32 = 1.0 / _rhs;

        Vec3 {
            e: [self.e[0] * k, self.e[1] * k, self.e[2] * k],
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + _rhs.e[0],
                self.e[1] + _rhs.e[1],
                self.e[2] + _rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {
        assert_eq!(
            Vec3::new(2.0, 4.0, 6.0) + Vec3::new(1.0, 1.0, 4.0),
            Vec3::new(3.0, 5.0, 10.0)
        );
    }

    #[test]
    fn test_vec3_multiply() {
        assert_eq!(Vec3::new(2.0, 3.0, 4.0) * 2.0, Vec3::new(4.0, 6.0, 8.0))
    }

    #[test]
    fn test_vec3_divide() {
        assert_eq!(Vec3::new(4.0, 6.0, 8.0) / 2.0, Vec3::new(2.0, 3.0, 4.0))
    }
}
