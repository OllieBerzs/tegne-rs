// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// 2 component vector

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use super::Vector3;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: impl Into<Self>) -> f32 {
        let o = other.into();
        self.x * o.x + self.y * o.y
    }

    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn unit(&self) -> Self {
        let scale = 1.0 / self.length();
        *self * if scale.is_infinite() { 0.0 } else { scale }
    }

    pub fn angle_between(&self, other: impl Into<Self>) -> f32 {
        let o = other.into();
        let cos = self.dot(o) / (self.length() * o.length());
        cos.acos().to_degrees()
    }

    pub const fn extend(&self, z: f32) -> Vector3 {
        Vector3::new(self.x, self.y, z)
    }

    pub const ZERO: Self = Self::new(0.0, 0.0);
}

impl From<[f32; 2]> for Vector2 {
    fn from(array: [f32; 2]) -> Self {
        Self::new(array[0], array[1])
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Add<Self> for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Self> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl AddAssign<Self> for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign<Self> for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod test {
    use super::Vector2;
    use super::Vector3;

    #[test]
    fn default() {
        let v = Vector2::default();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
    }

    #[test]
    fn new() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn from_array() {
        let v: Vector2 = [5.0, 6.0].into();
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 6.0);
    }

    #[test]
    fn dot() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(5.0, 6.0);
        assert_eq!(a.dot(b), 17.0);
    }

    #[test]
    fn length() {
        let v = Vector2::new(2.0, 4.0);
        assert_eq!(v.length(), 4.472_136);
    }

    #[test]
    fn unit() {
        let v = Vector2::new(3.0, 4.0);
        assert_eq!(v.unit(), Vector2::new(0.6, 0.8));
    }

    #[test]
    fn extend() {
        let v = Vector2::new(2.0, 5.5);
        assert_eq!(v.extend(4.7), Vector3::new(2.0, 5.5, 4.7));
    }

    #[test]
    fn operators() {
        let v1 = Vector2::new(2.0, 3.0);
        let v2 = Vector2::new(2.0, 8.0);
        assert_eq!(-v1, Vector2::new(-2.0, -3.0));
        assert_eq!(v1 + v2, Vector2::new(4.0, 11.0));
        assert_eq!(v1 - v2, Vector2::new(0.0, -5.0));
        assert_eq!(v1 * 4.0, Vector2::new(8.0, 12.0));
        assert_eq!(v2 / 2.0, Vector2::new(1.0, 4.0));
    }

    #[test]
    fn operators_assign() {
        let v = Vector2::new(2.0, 2.0);
        let mut add = Vector2::new(1.0, 3.0);
        let mut sub = Vector2::new(3.0, 5.0);
        let mut mul = Vector2::new(1.0, 3.0);
        let mut div = Vector2::new(4.0, 6.0);
        add += v;
        sub -= v;
        mul *= 2.0;
        div /= 2.0;
        assert_eq!(add, Vector2::new(3.0, 5.0));
        assert_eq!(sub, Vector2::new(1.0, 3.0));
        assert_eq!(mul, Vector2::new(2.0, 6.0));
        assert_eq!(div, Vector2::new(2.0, 3.0));
    }
}
