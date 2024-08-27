use crate::vec3::Vec3;

pub trait Lerp {
    #[allow(unused)]
    fn lerp(self, rhs: Self, fac: f64) -> Self;
}

impl Lerp for f64 {
    fn lerp(self, rhs: Self, fac: f64) -> Self {
        (1.0 - fac) * rhs + fac * self
    }
}

impl Lerp for Vec3 {
    fn lerp(self, rhs: Self, fac: f64) -> Self {
        (1.0 - fac) * rhs + fac * self
    }
}

pub fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        return linear.sqrt();
    }

    0.0
}

pub fn gamma_to_linear(gamma: f64) -> f64 {
    (gamma * gamma).clamp(0.0, 1.0)
}
