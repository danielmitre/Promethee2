use std::{
    fmt::{Debug, Display},
    mem::swap,
};

use super::*;
use itertools::izip;

pub(crate) struct Vanilla {}

impl Vanilla {
    fn flow<U, T, F>(criteria: &Criteria<U, T, F>, mut flow: Flow<U>) -> Flow<U>
    where
        U: Float + DivAssign + AddAssign + Copy + Display + Debug,
        T: ExactSizeIterator<Item = U> + Clone,
        F: ComparisonFunction<U> + Debug,
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

                *positive_flow += weight * positive;
                *negative_flow += weight * negative;
            }
        }

        flow
    }
}

impl Promethee for Vanilla {
    fn rank<U, T, F>(criterias: Vec<Criteria<U, T, F>>) -> Flow<U>
    where
        U: Float + DivAssign + AddAssign + Copy + Display + Debug,
        T: ExactSizeIterator<Item = U> + Clone,
        F: ComparisonFunction<U> + Debug,
    {
        // Used to normalize the criteria weights
        let mut total_weight = U::zero();
        for criteria in criterias.iter() {
            total_weight += criteria.weight;
        }

        let n = criterias.iter().map(|x| x.actions.len()).max().unwrap();

        let mut flow = Flow {
            positive_flow: vec![U::zero(); n],
            negative_flow: vec![U::zero(); n],
            net_flow: vec![U::zero(); n],
        };

        for mut criteria in criterias.into_iter() {
            criteria.weight /= total_weight;
            flow = Self::flow(&criteria, flow);
        }

        let denominator = U::from(n - 1).unwrap();
        for (positive, negative, net_flow) in izip!(
            flow.positive_flow.iter_mut(),
            flow.negative_flow.iter_mut(),
            flow.net_flow.iter_mut()
        ) {
            *positive /= denominator;
            *negative /= denominator;
            *net_flow = *positive - *negative;
        }

        flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS_FLOAT_CMP: f64 = 1e-9;

    fn float_cmp<T: Float>(left: T, right: T, abs_error: T) -> bool {
        T::max(left, right) - T::min(left, right) < abs_error
    }

    fn approx_eq_iter<'a, F: Float + 'a, T: Iterator<Item = &'a F>>(
        left: T,
        right: T,
    ) -> Option<(usize, &'a F, &'a F)> {
        for (pos, (l, r)) in left.zip(right).enumerate() {
            if !float_cmp(*l, *r, F::from(EPS_FLOAT_CMP).unwrap()) {
                return Some((pos, l, r));
            }
        }
        None
    }

    fn assert_approx_eq<F: Float + Display>(left: Flow<F>, right: Flow<F>) {
        if let Some((pos, l, r)) =
            approx_eq_iter(left.positive_flow.iter(), right.positive_flow.iter())
        {
            panic!(
                "positive_flow differs at position {}. left: {}, right: {}\n",
                pos, *l, *r
            );
        }
        if let Some((pos, l, r)) =
            approx_eq_iter(left.negative_flow.iter(), right.negative_flow.iter())
        {
            panic!(
                "negative_flow differs at position {}. left: {}, right: {}.\n",
                pos, *l, *r
            );
        }
        if let Some((pos, l, r)) = approx_eq_iter(left.net_flow.iter(), right.net_flow.iter()) {
            panic!(
                "net_flow differs at position {}. left: {}, right: {}",
                pos, *l, *r
            );
        }
    }

    /// Example taken from https://youtu.be/xe2XgGrI0Sg
    #[test]
    fn youtube_example() {
        let price = Criteria {
            actions: vec![250.0, 200.0, 300.0, 275.0].into_iter(),
            weight: 0.35,
            function: LinearFunction { p: 100.0 },
            goal: Goal::Min,
        };

        let storage = Criteria {
            actions: vec![16.0, 16.0, 32.0, 32.0].into_iter(),
            weight: 0.25,
            function: LinearFunction { p: 16.0 },
            goal: Goal::Max,
        };

        let camera = Criteria {
            actions: vec![12.0, 8.0, 16.0, 8.0].into_iter(),
            weight: 0.25,
            function: LinearFunction { p: 8.0 },
            goal: Goal::Max,
        };

        let looks = Criteria {
            actions: vec![5.0, 3.0, 4.0, 2.0].into_iter(),
            weight: 0.15,
            function: LinearFunction { p: 3.0 },
            goal: Goal::Max,
        };

        let expected = Flow {
            positive_flow: vec![0.2708333333, 0.2791666667, 0.425, 0.1958333333],
            negative_flow: vec![0.2666666667, 0.3416666667, 0.2208333333, 0.3416666667],
            net_flow: vec![0.004166666667, -0.0625, 0.2041666667, -0.1458333333],
        };

        assert_approx_eq(expected, Vanilla::rank(vec![price, storage, camera, looks]));
    }

    #[test]
    fn single_min_linear() {
        let erosao = Criteria {
            actions: vec![4.8, 3.4, 3.8, 4.5].into_iter(),
            weight: 1.0,
            function: LinearFunction { p: 5.0 },
            goal: Goal::Min,
        };

        let expected = Flow {
            positive_flow: vec![0.00, 0.1933333333, 0.1133333333, 0.02],
            negative_flow: vec![0.18, 0.00, 0.0266666667, 0.12],
            net_flow: vec![-0.18, 0.1933333333, 0.0866666667, -0.1],
        };

        assert_approx_eq(expected, Vanilla::rank(vec![erosao]));
    }

    #[test]
    fn max_linear_criteria() {
        let erosao = Criteria {
            actions: vec![4.8, 3.4, 3.8, 4.5].into_iter(),
            weight: 2.0,
            function: LinearFunction { p: 5.0 },
            goal: Goal::Min,
        };

        let infpop = Criteria {
            actions: vec![300.0, 155.0, 200.0, 280.0].into_iter(),
            weight: 1.0,
            function: LinearFunction { p: 500.0 },
            goal: Goal::Min,
        };

        let prod = Criteria {
            actions: vec![6.2, 4.1, 5.0, 4.0].into_iter(),
            weight: 4.0,
            function: LinearFunction { p: 7.0 },
            goal: Goal::Max,
        };

        let rhcp = Criteria {
            actions: vec![2.2, 0.5, 0.7, 2.5].into_iter(),
            weight: 3.0,
            function: LinearFunction { p: 2.5 },
            goal: Goal::Max,
        };

        let expected = Flow {
            positive_flow: vec![0.2327619048, 0.06157142857, 0.07885714286, 0.1693333333],
            negative_flow: vec![0.06566666667, 0.2131428571, 0.1631904762, 0.1005238095],
            net_flow: vec![0.1670952381, -0.1515714286, -0.08433333333, 0.06880952381],
        };

        assert_approx_eq(expected, Vanilla::rank(vec![erosao, infpop, prod, rhcp]));
    }
}
