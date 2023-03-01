pub(crate) mod vanilla;

pub(crate) use crate::function::*;
pub(crate) use num::Float;
use std::fmt::{Debug, Display};
pub(crate) use std::ops::{AddAssign, DivAssign};

#[derive(PartialEq, Debug)]
pub(crate) struct Flow<U> {
    positive_flow: Vec<U>,
    negative_flow: Vec<U>,
    net_flow: Vec<U>,
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct Criteria<U, T, F>
where
    U: Float + DivAssign + AddAssign + Copy,
    T: ExactSizeIterator<Item = U> + Clone,
    F: ComparisonFunction<U>,
{
    pub(crate) actions: T,
    pub(crate) weight: U,
    pub(crate) function: F,
    pub(crate) goal: Goal,
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Goal {
    Max,
    Min,
}

pub(crate) trait Promethee {
    fn rank<U, T, F>(criterias: Vec<Criteria<U, T, F>>) -> Flow<U>
    where
        U: Float + DivAssign + AddAssign + Copy + Display + Debug,
        T: ExactSizeIterator<Item = U> + Clone,
        F: ComparisonFunction<U> + Debug;
}
