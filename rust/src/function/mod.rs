use std::{
    f64::consts,
    ops::{Div, Mul, Neg, Sub},
};

use num_traits::Pow;

use super::*;

#[derive(Clap, Debug)]
pub(crate) struct UsualFunction {}

#[derive(Clap, Debug)]
pub(crate) struct QuasiFunction {
    pub(crate) l: f64,
}

#[derive(Clap, Debug)]
pub(crate) struct LinearFunction {
    #[clap()]
    pub(crate) m: f64,
}

#[derive(Clap, Debug)]
pub(crate) struct LevelFunction {
    pub(crate) weak_treshold: f64,
    pub(crate) weak_area: f64,
}

#[derive(Clap, Debug)]
pub(crate) struct LinearWithIndeferenceFunction {
    pub(crate) indiference_threshold: f64,
    pub(crate) linear_area: f64,
}

#[derive(Clap, Debug)]
pub(crate) struct GaussianFunction {
    pub(crate) std_dev: f64,
}

pub(crate) trait ComparisonFunction<T> {
    fn compare(&self, arg1: T, arg2: T) -> T;
}

impl<T> ComparisonFunction<T> for PreferenceFunction
where
    T: From<f64>
        + Neg<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Pow<T, Output = T>
        + PartialOrd
        + std::marker::Copy,
{
    fn compare(&self, arg1: T, arg2: T) -> T {
        match self {
            PreferenceFunction::Usual(f) => f.compare(arg1, arg2),
            PreferenceFunction::Quasi(f) => f.compare(arg1, arg2),
            PreferenceFunction::Linear(f) => f.compare(arg1, arg2),
            PreferenceFunction::Level(f) => f.compare(arg1, arg2),
            PreferenceFunction::LinearWithIndeference(f) => f.compare(arg1, arg2),
            PreferenceFunction::Gaussian(f) => f.compare(arg1, arg2),
        }
    }
}

impl<T> ComparisonFunction<T> for UsualFunction
where
    T: From<f64> + Sub<Output = T> + PartialOrd,
{
    fn compare(&self, arg1: T, arg2: T) -> T {
        let diff = arg1 - arg2;
        let zero = T::from(0.0);
        if diff <= zero {
            return zero;
        }
        T::from(1.0)
    }
}

impl<T> ComparisonFunction<T> for QuasiFunction
where
    T: From<f64> + Sub<Output = T> + PartialOrd,
{
    fn compare(&self, arg1: T, arg2: T) -> T {
        let diff = arg1 - arg2;
        let zero = T::from(0.0);
        if diff <= zero {
            return zero;
        }
        if diff < T::from(self.l) {
            return zero;
        }
        T::from(1.0)
    }
}

impl<T> ComparisonFunction<T> for LinearFunction
where
    T: From<f64> + Sub<Output = T> + PartialOrd + Div<Output = T>,
{
    fn compare(&self, arg1: T, arg2: T) -> T {
        let diff = arg1 - arg2;
        let zero = T::from(0.0);
        if diff <= zero {
            return zero;
        }
        let p = T::from(self.m);
        if diff < p {
            return diff / p;
        }
        T::from(1.0)
    }
}

impl<T> ComparisonFunction<T> for LevelFunction
where
    T: From<f64> + Sub<Output = T> + PartialOrd,
{
    fn compare(&self, arg1: T, arg2: T) -> T {
        let diff = arg1 - arg2;
        let zero = T::from(0.0);
        if diff <= zero {
            return zero;
        }
        if diff <= T::from(self.weak_treshold) {
            return zero;
        }
        if diff <= T::from(self.weak_treshold + self.weak_area) {
            return T::from(0.5);
        }
        T::from(1.0)
    }
}

impl<T> ComparisonFunction<T> for LinearWithIndeferenceFunction
where
    T: From<f64> + Sub<Output = T> + PartialOrd + Div<Output = T>,
{
    fn compare(&self, arg1: T, arg2: T) -> T {
        let diff = arg1 - arg2;
        let zero = T::from(0.0);
        if diff <= zero {
            return zero;
        }

        if diff <= T::from(self.indiference_threshold) {
            return zero;
        }

        if diff <= T::from(self.indiference_threshold + self.linear_area) {
            return (diff - T::from(self.indiference_threshold)) / T::from(self.linear_area);
        }
        T::from(1.0)
    }
}

impl<T> ComparisonFunction<T> for GaussianFunction
where
    T: From<f64>
        + Sub<Output = T>
        + Neg<Output = T>
        + PartialOrd
        + Mul<Output = T>
        + Div<Output = T>
        + Pow<T, Output = T>
        + Copy,
{
    fn compare(&self, arg1: T, arg2: T) -> T {
        let diff = arg1 - arg2;
        let zero = T::from(0.0);
        if diff <= zero {
            return zero;
        }

        T::from(1.0)
            - T::pow(
                T::from(consts::E),
                -(diff * diff) / T::from(2.0 * self.std_dev * self.std_dev),
            )
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::*;
    fn assert_eq_float(left: f64, right: f64, eps: f64) {
        if (left - right).abs() > eps {
            panic!("left={:?} right={:?}", left, right)
        }
    }

    #[test]
    fn usual_function() {
        let a = UsualFunction {};
        assert_eq!(0.00, a.compare(0.0, 0.0));
        assert_eq!(1.00, a.compare(1.0, 0.0));
        assert_eq!(1.00, a.compare(2.0, 0.0));
        assert_eq!(0.00, a.compare(0.0, 1.0));
        assert_eq!(0.00, a.compare(0.0, 2.0));
    }

    #[test]
    fn quasi_function() {
        let a = QuasiFunction { l: 4.0 };
        assert_eq!(0.00, a.compare(0.0, 0.0));
        assert_eq!(0.00, a.compare(1.0, 0.0));
        assert_eq!(0.00, a.compare(2.0, 0.0));
        assert_eq!(0.00, a.compare(3.0, 0.0));
        assert_eq!(1.00, a.compare(4.0, 0.0));
        assert_eq!(1.00, a.compare(5.0, 0.0));
    }

    #[test]
    fn linear_function() {
        let a = LinearFunction { m: 4.0 };
        assert_eq!(0.00, a.compare(0.0, 0.0));
        assert_eq!(0.25, a.compare(1.0, 0.0));
        assert_eq!(0.50, a.compare(2.0, 0.0));
        assert_eq!(0.75, a.compare(3.0, 0.0));
        assert_eq!(1.00, a.compare(4.0, 0.0));
        assert_eq!(1.00, a.compare(5.0, 0.0));
    }

    #[test]
    fn level_function() {
        let a = LevelFunction {
            weak_treshold: 2.0,
            weak_area: 2.0,
        };
        assert_eq!(0.0, a.compare(0.0, 0.0));
        assert_eq!(0.0, a.compare(1.0, 0.0));
        assert_eq!(0.0, a.compare(2.0, 0.0));
        assert_eq!(0.5, a.compare(3.0, 0.0));
        assert_eq!(0.5, a.compare(4.0, 0.0));
        assert_eq!(1.0, a.compare(5.0, 0.0));
    }

    #[test]
    fn linear_with_indiference_function() {
        let a = LinearWithIndeferenceFunction {
            indiference_threshold: 2.0,
            linear_area: 2.0,
        };
        assert_eq!(0.0, a.compare(0.0, 0.0));
        assert_eq!(0.0, a.compare(1.0, 0.0));
        assert_eq!(0.0, a.compare(2.0, 0.0));
        assert_eq!(0.5, a.compare(3.0, 0.0));
        assert_eq!(1.0, a.compare(4.0, 0.0));
        assert_eq!(1.0, a.compare(5.0, 0.0));
    }

    #[test]
    fn gaussian_function() {
        let a = GaussianFunction { std_dev: 1.0 };
        assert_eq!(0.0, a.compare(0.0, 0.0));
        assert_eq_float(0.117503097, a.compare(0.5, 0.0), 1e-9);
        assert_eq_float(0.393469340, a.compare(1.0, 0.0), 1e-9);
        assert_eq_float(0.864664716, a.compare(2.0, 0.0), 1e-9);
    }
}
