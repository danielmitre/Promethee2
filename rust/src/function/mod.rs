use super::*;
use num::{Float};

#[derive(Clap, Debug)]
pub(crate) struct LinearFunction {
  #[clap()]
  pub(crate) p: f64,
}

pub(crate) trait ComparisonFunction<T: Float> {
  fn compare(&self, arg1: T, arg2: T) -> T;
}

impl <T: Float> ComparisonFunction<T> for PreferenceFunction {
  fn compare(&self, arg1: T, arg2: T) -> T {
    match self {
      PreferenceFunction::Linear(f) => {
        f.compare(arg1, arg2)
      },
    }
  }
}

impl <T: Float> ComparisonFunction<T> for LinearFunction {
  fn compare(&self, arg1: T, arg2: T) -> T {
    let diff = arg1 - arg2;
    if diff < T::zero() {
      return T::zero();
    }

    let p = T::from(self.p).unwrap();
    if diff < p {
      return diff / p;
    }
    
    T::one()
  }
}