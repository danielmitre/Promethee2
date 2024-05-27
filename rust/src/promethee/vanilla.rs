use std::{
    cmp::Ordering,
    fmt::Debug,
    mem::swap,
    ops::{Add, Div, Mul, Neg, Sub},
};

use super::*;
use itertools::{izip, Itertools};
use num_traits::Pow;

pub(crate) struct Vanilla {
    divide_by_alternatives: bool,
}

impl Vanilla {
    pub fn new(divide_by_alternatives: bool) -> Self {
        Self {
            divide_by_alternatives,
        }
    }

    fn flow<T, I, F>(&mut self, criteria: &Criteria<T, I, F>, mut flow: Flow<T>) -> Flow<T>
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
        F: ComparisonFunction<T> + Debug,
    {
        let actions = criteria.actions.clone().collect::<Vec<_>>();
        let weight = criteria.weight;

        for (pixel, positive_flow, negative_flow) in izip!(
            actions.iter(),
            flow.positive_flow.iter_mut(),
            flow.negative_flow.iter_mut()
        ) {
            for other in actions.iter() {
                let mut positive = criteria.function.compare(*pixel, *other);
                let mut negative = criteria.function.compare(*other, *pixel);

                if criteria.goal == Goal::Min {
                    swap(&mut positive, &mut negative);
                }

                *positive_flow = *positive_flow + (weight * positive);
                *negative_flow = *negative_flow + (weight * negative);
            }
        }

        flow
    }
}

impl Promethee for Vanilla {
    fn rank<T, I, F>(mut self, criterias: Vec<Criteria<T, I, F>>) -> (Flow<T>, Vec<usize>)
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
        F: ComparisonFunction<T> + Debug,
    {
        // Used to normalize the criteria weights
        let mut total_weight = T::from(0.0);
        for criteria in criterias.iter() {
            total_weight = total_weight + criteria.weight;
        }

        let n = criterias.iter().map(|x| x.actions.len()).max().unwrap();

        let mut flow = Flow {
            positive_flow: vec![T::from(0.0); n],
            negative_flow: vec![T::from(0.0); n],
            net_flow: vec![T::from(0.0); n],
        };

        for mut criteria in criterias.into_iter() {
            criteria.weight = criteria.weight / total_weight;
            flow = self.flow(&criteria, flow);
        }

        let denominator = T::from((n - 1) as f64);
        for (positive, negative, net_flow) in izip!(
            flow.positive_flow.iter_mut(),
            flow.negative_flow.iter_mut(),
            flow.net_flow.iter_mut()
        ) {
            if self.divide_by_alternatives {
                *positive = *positive / denominator;
                *negative = *negative / denominator;
            }
            *net_flow = *positive - *negative;
        }

        let mut rank = (0..n).collect_vec();
        rank.sort_by(|a, b| {
            if flow.net_flow[*a] > flow.net_flow[*b] {
                return Ordering::Less;
            }
            if flow.net_flow[*a] < flow.net_flow[*b] {
                return Ordering::Greater;
            }
            Ordering::Equal
        });
        (flow, rank)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        ops::{Add, Div, Mul, Rem, Sub},
        vec,
    };

    use crate::PreferenceFunction;

    use super::*;

    fn eq_float(left: f64, right: f64, abs_error: f64) -> bool {
        (left - right).abs() < abs_error
    }

    fn eq_floats<'a, Iter: Iterator<Item = f64>>(
        left: Iter,
        right: Iter,
        eps: f64,
    ) -> Option<(usize, f64, f64)> {
        for (pos, (l, r)) in left.zip(right).enumerate() {
            if !eq_float(l, r, eps) {
                return Some((pos, l, r));
            }
        }
        None
    }

    fn assert_approx_eq<T: Copy + Into<f64>>(left: Flow<T>, right: Flow<T>, eps: f64) {
        // if let Some((pos, l, r)) = eq_floats(
        //     left.positive_flow.clone().into_iter().map(T::into),
        //     right.positive_flow.clone().into_iter().map(T::into),
        //     eps,
        // ) {
        //     panic!(
        //         "positive_flow differs at position {}. left: {}, right: {}\n",
        //         pos, l, r
        //     );
        // }
        // if let Some((pos, l, r)) = eq_floats(
        //     left.negative_flow.clone().into_iter().map(T::into),
        //     right.negative_flow.clone().into_iter().map(T::into),
        //     eps,
        // ) {
        //     panic!(
        //         "negative_flow differs at position {}. left: {}, right: {}.\n",
        //         pos, l, r
        //     );
        // }
        if let Some((pos, l, r)) = eq_floats(
            left.net_flow.clone().into_iter().map(T::into),
            right.net_flow.clone().into_iter().map(T::into),
            eps,
        ) {
            panic!(
                "net_flow differs at position {}. left: {}, right: {}",
                pos, l, r
            );
        }
    }

    /// Example taken from https://youtu.be/xe2XgGrI0Sg
    #[test]
    fn youtube_example() {
        let price = Criteria {
            actions: vec![250.0, 200.0, 300.0, 275.0].into_iter(),
            weight: 0.35,
            function: LinearFunction { m: 100.0 },
            goal: Goal::Min,
        };

        let storage = Criteria {
            actions: vec![16.0, 16.0, 32.0, 32.0].into_iter(),
            weight: 0.25,
            function: LinearFunction { m: 16.0 },
            goal: Goal::Max,
        };

        let camera = Criteria {
            actions: vec![12.0, 8.0, 16.0, 8.0].into_iter(),
            weight: 0.25,
            function: LinearFunction { m: 8.0 },
            goal: Goal::Max,
        };

        let looks = Criteria {
            actions: vec![5.0, 3.0, 4.0, 2.0].into_iter(),
            weight: 0.15,
            function: LinearFunction { m: 3.0 },
            goal: Goal::Max,
        };

        let want_rank = vec![2, 0, 1, 3];

        let want_flow = Flow {
            positive_flow: vec![0.2708333333, 0.2791666667, 0.425, 0.1958333333],
            negative_flow: vec![0.2666666667, 0.3416666667, 0.2208333333, 0.3416666667],
            net_flow: vec![0.004166666667, -0.0625, 0.2041666667, -0.1458333333],
        };

        let promethee = Vanilla::new(true);

        let (got_flow, got_rank) = promethee.rank(vec![price, storage, camera, looks]);
        assert_approx_eq(want_flow, got_flow, 1e-9);
        assert_eq!(want_rank, got_rank);
    }

    #[test]
    fn single_min_linear() {
        let erosao = Criteria {
            actions: vec![4.8, 3.4, 3.8, 4.5].into_iter(),
            weight: 1.0,
            function: LinearFunction { m: 5.0 },
            goal: Goal::Min,
        };

        let want_flow = Flow {
            positive_flow: vec![0.00, 0.1933333333, 0.1133333333, 0.02],
            negative_flow: vec![0.18, 0.00, 0.0266666667, 0.12],
            net_flow: vec![-0.18, 0.1933333333, 0.0866666667, -0.1],
        };

        let want_rank = vec![1, 2, 3, 0];

        let promethee = Vanilla::new(true);
        let (got_flow, got_rank) = promethee.rank(vec![erosao]);

        assert_eq!(got_rank, want_rank);
        assert_approx_eq(want_flow, got_flow, 1e-9);
    }

    #[test]
    fn max_linear_criteria() {
        let erosao = Criteria {
            actions: vec![4.8, 3.4, 3.8, 4.5].into_iter(),
            weight: 2.0,
            function: LinearFunction { m: 5.0 },
            goal: Goal::Min,
        };

        let infpop = Criteria {
            actions: vec![300.0, 155.0, 200.0, 280.0].into_iter(),
            weight: 1.0,
            function: LinearFunction { m: 500.0 },
            goal: Goal::Min,
        };

        let prod = Criteria {
            actions: vec![6.2, 4.1, 5.0, 4.0].into_iter(),
            weight: 4.0,
            function: LinearFunction { m: 7.0 },
            goal: Goal::Max,
        };

        let rhcp = Criteria {
            actions: vec![2.2, 0.5, 0.7, 2.5].into_iter(),
            weight: 3.0,
            function: LinearFunction { m: 2.5 },
            goal: Goal::Max,
        };

        let want_flow = Flow {
            positive_flow: vec![0.2327619048, 0.06157142857, 0.07885714286, 0.1693333333],
            negative_flow: vec![0.06566666667, 0.2131428571, 0.1631904762, 0.1005238095],
            net_flow: vec![0.1670952381, -0.1515714286, -0.08433333333, 0.06880952381],
        };

        let want_rank = vec![0, 3, 2, 1];

        let promethee = Vanilla::new(true);
        let (got_flow, got_rank) = promethee.rank(vec![erosao, infpop, prod, rhcp]);

        assert_eq!(want_rank, got_rank);
        assert_approx_eq(want_flow, got_flow, 1e-9);
    }

    #[derive(Copy, Clone, PartialEq, PartialOrd)]
    struct Rounded {
        n: f64,
    }

    impl From<f64> for Rounded {
        fn from(value: f64) -> Self {
            Self { n: value }
        }
    }

    impl Into<f64> for Rounded {
        fn into(self) -> f64 {
            self.n
        }
    }

    impl Add for Rounded {
        type Output = Rounded;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                n: (1000.0 * (self.n + rhs.n)).round() / 1000.0,
            }
        }
    }

    impl Rem for Rounded {
        type Output = Rounded;

        fn rem(self, rhs: Self) -> Self::Output {
            Self { n: self.n % rhs.n }
        }
    }

    impl Div for Rounded {
        type Output = Rounded;

        fn div(self, rhs: Self) -> Self::Output {
            Self {
                n: (1000.0 * self.n / rhs.n).round() / 1000.0,
            }
        }
    }

    impl Mul for Rounded {
        type Output = Rounded;

        fn mul(self, rhs: Self) -> Self::Output {
            Self {
                n: (1000.0 * self.n * rhs.n).round() / 1000.0,
            }
        }
    }

    impl Sub for Rounded {
        type Output = Rounded;

        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                n: (1000.0 * (self.n - rhs.n)).round() / 1000.0,
            }
        }
    }

    impl Neg for Rounded {
        type Output = Rounded;

        fn neg(self) -> Self::Output {
            Self { n: -self.n }
        }
    }

    impl Pow<Rounded> for Rounded {
        type Output = Rounded;

        fn pow(self, rhs: Rounded) -> Self::Output {
            Self {
                n: (1000.0 * self.n.pow(rhs.n)) / 1000.0,
            }
        }
    }

    /// Example taken from https://pubsonline.informs.org/doi/pdf/10.1287/mnsc.31.6.647
    #[test]
    fn paper_example() {
        let f1 = Criteria {
            actions: vec![80.0, 65.0, 83.0, 40.0, 52.0, 94.0]
                .into_iter()
                .map(Rounded::from),
            weight: Rounded::from(1.0),
            function: PreferenceFunction::Quasi(QuasiFunction { l: 10.0 }),
            goal: Goal::Min,
        };

        let f2 = Criteria {
            actions: vec![90.0, 58.0, 60.0, 80.0, 72.0, 96.0]
                .into_iter()
                .map(Rounded::from),
            weight: Rounded::from(1.0),
            function: PreferenceFunction::Linear(LinearFunction { m: 30.0 }),
            goal: Goal::Max,
        };

        let f3 = Criteria {
            actions: vec![60.0, 20.0, 40.0, 100.0, 60.0, 70.0]
                .into_iter()
                .map(Rounded::from),
            weight: Rounded::from(1.0),
            function: PreferenceFunction::LinearWithIndeference(LinearWithIndeferenceFunction {
                indiference_threshold: 5.0,
                linear_area: 45.0,
            }),
            goal: Goal::Min,
        };

        let f4 = Criteria {
            actions: vec![5.4, 9.7, 7.2, 7.5, 2.0, 3.6]
                .into_iter()
                .map(Rounded::from),
            weight: Rounded::from(1.0),
            function: PreferenceFunction::Level(LevelFunction {
                weak_treshold: 1.0,
                weak_area: 5.0,
            }),
            goal: Goal::Min,
        };

        let f5 = Criteria {
            actions: vec![8.0, 1.0, 4.0, 7.0, 3.0, 5.0]
                .into_iter()
                .map(Rounded::from),
            weight: Rounded::from(1.0),
            function: PreferenceFunction::Usual(UsualFunction {}),
            goal: Goal::Min,
        };

        let f6 = Criteria {
            actions: vec![5.0, 1.0, 7.0, 10.0, 8.0, 6.0]
                .into_iter()
                .map(Rounded::from),
            weight: Rounded::from(1.0),
            function: PreferenceFunction::Gaussian(GaussianFunction { std_dev: 5.0 }),
            goal: Goal::Max,
        };

        let want_flow = Flow {
            positive_flow: vec![1.099, 1.980, 1.234, 1.644, 2.274, 1.500]
                .into_iter()
                .map(Rounded::from)
                .collect(),
            negative_flow: vec![1.827, 1.895, 1.681, 1.786, 0.808, 1.744]
                .into_iter()
                .map(Rounded::from)
                .collect(),
            net_flow: vec![-0.728, 0.085, -0.447, -0.102, 1.466, -0.274]
                .into_iter()
                .map(Rounded::from)
                .collect(),
        };

        let want_rank = vec![4, 1, 3, 5, 2, 0];

        let promethee = Vanilla::new(false);
        let (got_flow, got_rank) = promethee.rank(vec![f1, f2, f3, f4, f5, f6]);

        assert_eq!(want_rank, got_rank);
        assert_approx_eq(want_flow, got_flow, 1e-2);
    }
}
