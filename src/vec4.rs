use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, RangeInclusive, Sub, SubAssign,
};

use rand::{thread_rng, Rng};

use crate::{utils::linear_to_gamma, Vec3};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4 {
    #[allow(unused)]
    const UP: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    #[allow(unused)]
    const DOWN: Self = Self::new(0.0, -1.0, 0.0, 0.0);
    #[allow(unused)]
    const LEFT: Self = Self::new(-1.0, 0.0, 0.0, 0.0);
    #[allow(unused)]
    const RIGHT: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    #[allow(unused)]
    const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    #[allow(unused)]
    const FORWARD: Self = Self::new(0.0, 0.0, 1.0, 0.0);
    #[allow(unused)]
    const BACKWARD: Self = Self::new(0.0, 0.0, -1.0, 0.0);

    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub const fn splat(val: f64) -> Self {
        Self {
            x: val,
            y: val,
            z: val,
            w: val,
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn truncate(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn to_gamma(self) -> ColorRGBA {
        ColorRGBA::new(
            linear_to_gamma(self.x),
            linear_to_gamma(self.y),
            linear_to_gamma(self.z),
            linear_to_gamma(self.w),
        )
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s) && (self.w.abs() < s)
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self::new(
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
        )
    }

    pub fn random_range(range: RangeInclusive<f64>) -> Self {
        let mut rng = thread_rng();
        Self::new(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        )
    }

    pub fn random_unit_vector() -> Self {
        Self::random().normalized()
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl MulAssign<Vec4> for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<f64> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl DivAssign<Vec4> for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Div<f64> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl DivAssign<f64> for Vec4 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Mul<Vec4> for f64 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs * self
    }
}

#[allow(unused)]
pub type Point4 = Vec4;

#[allow(unused)]
pub type ColorRGBA = Vec4;
