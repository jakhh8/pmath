use std::ops::RangeInclusive;

pub trait RangeExtension {
    #[allow(unused)]
    fn surrounds(&self, value: f64) -> bool;

    #[allow(unused)]
    fn clamp(&self, value: f64) -> f64;
}

impl RangeExtension for RangeInclusive<f64> {
    fn surrounds(&self, value: f64) -> bool {
        *self.start() < value && value < *self.end()
    }

    fn clamp(&self, value: f64) -> f64 {
        value.max(*self.start()).min(*self.end())
    }
}
