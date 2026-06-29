use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use std::ops::MulAssign;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Vec2i {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Add for Vec2i{
    type Output = Vec2i;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2i{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl MulAssign for Vec2i {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<i32> for Vec2i{
    type Output = Vec2i;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2i {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Sub for Vec2i{
    type Output = Vec2i;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2i{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2i {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y}
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Mat3 {
    pub r1: [f32;3] ,
    pub r2: [f32;3],
    pub r3: [f32;3],
}

impl Mat3{
    pub const fn new(r1: [f32;3], r2: [f32;3], r3: [f32;3]) -> Self{ Self{r1, r2, r3}}
}

impl Mul<Mat3> for Mat3{
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let r1 = [self.r1[0] * rhs.r1[0] + self.r1[1] * rhs.r2[0] + self.r1[2] * rhs.r3[0],
                         self.r1[0] * rhs.r1[1] + self.r1[1] * rhs.r2[1] + self.r1[2] * rhs.r3[1],
                         self.r1[0] * rhs.r1[2] + self.r1[1] * rhs.r2[2] + self.r1[2] * rhs.r3[2]];

        let r2 = [self.r2[0] * rhs.r1[0] + self.r2[1] * rhs.r2[0] + self.r2[2] * rhs.r3[0],
                         self.r2[0] * rhs.r1[1] + self.r2[1] * rhs.r2[1] + self.r2[2] * rhs.r3[1],
                         self.r2[0] * rhs.r1[2] + self.r2[1] * rhs.r2[2] + self.r2[2] * rhs.r3[2]];

        let r3 = [self.r3[0] * rhs.r1[0] + self.r3[1] * rhs.r2[0] + self.r3[2] * rhs.r3[0],
                         self.r3[0] * rhs.r1[1] + self.r3[1] * rhs.r2[1] + self.r3[2] * rhs.r3[1],
                         self.r3[0] * rhs.r1[2] + self.r3[1] * rhs.r2[2] + self.r3[2] * rhs.r3[2]];
        Mat3{
            r1,
            r2,
            r3
        }
    }
}

impl Mul<Vec3> for Mat3{
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3{
            x: self.r1[0] * rhs.x + self.r1[1] * rhs.y + self.r1[2] * rhs.z,
            y: self.r2[0] * rhs.x + self.r2[1] * rhs.y + self.r2[2] * rhs.z,
            z: self.r3[0] * rhs.x + self.r3[1] * rhs.y + self.r3[2] * rhs.z,
        }
    }
}

impl PartialEq for Mat3
{
    fn eq(&self, other: &Self) -> bool {
        self.r1 == other.r1 && self.r2 == other.r2 && self.r3 == other.r3
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl fmt::Display for Mat3
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[[\n{:.1}, {:.1}, {:.1}\n{:.1}, {:.1}, {:.1}\n{:.1}, {:.1}, {:.1}\n]]",
               self.r1[0], self.r1[1], self.r1[2],
               self.r2[0], self.r2[1], self.r2[2],
               self.r3[0], self.r3[1], self.r3[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat3_print(){
        let m = Mat3{
            r1: [1.0, 2.0, 3.0],
            r2: [4.0, 5.0, 6.0],
            r3: [7.0, 8.0, 9.0],
        };
        assert_eq!(format!("{}", m), "[[\n1.0, 2.0, 3.0\n4.0, 5.0, 6.0\n7.0, 8.0, 9.0\n]]");
    }

    #[test]
    fn test_mat3_multiplication(){
        let m1 = Mat3{
            r1: [1.0, 2.0, 3.0],
            r2: [4.0, 5.0, 6.0],
            r3: [7.0, 8.0, 9.0],
        };
        let m2 = Mat3{
            r1: [1.0, 2.0, 3.0],
            r2: [4.0, 5.0, 6.0],
            r3: [7.0, 8.0, 9.0],
        };

        let m = m1 * m2;
        let m_ref = Mat3{
            r1: [30.0, 36.0, 42.0],
            r2: [66.0, 81.0, 96.0],
            r3: [102.0, 126.0, 150.0]
        };
        assert_eq!(m, m_ref);
    }
    #[test]
    fn test_mat3_vec_multiplication() {
        let m = Mat3{
            r1: [1.0, 2.0, 3.0],
            r2: [4.0, 5.0, 6.0],
            r3: [7.0, 8.0, 9.0],
        };

        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let v = m * v;
        let v_ref = Vec3{ x: 14.0, y: 32.0, z:50.0};

        assert_eq!(v, v_ref);
    }
}