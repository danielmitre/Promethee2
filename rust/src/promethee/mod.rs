pub(crate) mod vanilla;
use num_traits::Pow;

pub(crate) use crate::function::*;
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(PartialEq, Debug)]
pub(crate) struct Flow<U> {
    positive_flow: Vec<U>,
    negative_flow: Vec<U>,
    net_flow: Vec<U>,
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct Criteria<T, I, F>
where
    T: From<f64>
        + Neg<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Pow<T, Output = T>
        + PartialOrd
        + std::marker::Copy,
    I: ExactSizeIterator<Item = T> + Clone,
    F: ComparisonFunction<T>,
{
    pub(crate) actions: I,
    pub(crate) weight: T,
    pub(crate) function: F,
    pub(crate) goal: Goal,
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Goal {
    Max,
    Min,
}

pub(crate) trait Promethee {
    fn rank<T, I, F>(self, criterias: Vec<Criteria<T, I, F>>) -> (Flow<T>, Vec<usize>)
    where
        T: From<f64>
            + Neg<Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Div<Output = T>
            + Mul<Output = T>
            + Pow<T, Output = T>
            + PartialOrd
            + std::marker::Copy,
        I: ExactSizeIterator<Item = T> + Clone,
        F: ComparisonFunction<T> + Debug;
}
