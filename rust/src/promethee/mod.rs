pub(crate) mod vanilla;

pub(crate) use crate::function::*;
pub(crate) use num::{Float};
pub(crate) use std::ops::{DivAssign, AddAssign};

pub(crate) struct Criteria<U, T, F>
where
  U: Float + DivAssign + AddAssign + Copy,
  T: ExactSizeIterator<Item=U> + Clone,
  F: ComparisonFunction<U>, {
  pub(crate) pixels: T,
  pub(crate) weight: U,
  pub(crate) function: F,
}

pub(crate) trait Promethee {
  fn rank<U, T, F, R>(criteria: Criteria<U, T, F>, flow: R)
  where
    U: Float + DivAssign + AddAssign + Copy,
    T: ExactSizeIterator<Item=U> + Clone,
    F: ComparisonFunction<U>,
    R: RandomAccesser<U>;
}

pub(crate) trait RandomAccesser<F: Float> {
  fn get(&self, pos: usize) -> F;
  fn set(&mut self, pos: usize, new: F);
  fn add(&mut self, pos: usize, additional: F) {
    self.set(pos, self.get(pos) + additional)
  }
  fn sub(&mut self, pos: usize, subtract: F) {
    self.add(pos, -subtract)
  }
  fn div(&mut self, pos: usize, denominator: F) {
    self.set(pos, self.get(pos) / denominator)
  }
}

impl <F: Float> RandomAccesser<F> for &mut [F] {
  fn get(&self, pos: usize) -> F {
    self[pos]
  }

  fn set(&mut self, pos: usize, new: F) {
    self[pos] = new;
  }
}

impl <F: Float> RandomAccesser<F> for Vec<F> {
  fn get(&self, pos: usize) -> F {
    self[pos]
  }

  fn set(&mut self, pos: usize, new: F) {
    self[pos] = new; 
  }
}