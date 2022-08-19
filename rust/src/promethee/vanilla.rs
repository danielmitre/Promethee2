use super::*;

pub(crate) struct Vanilla {}

impl Promethee for Vanilla {
    fn rank<U, T, F, R>(criteria: Criteria<U, T, F>, mut flow: R)
    where
        U: Float + DivAssign + AddAssign + Copy,
        T: ExactSizeIterator<Item = U> + Clone,
        F: ComparisonFunction<U>,
        R: RandomAccesser<U>,
    {
        // Compute area, positive and negative flow
        let area = U::from(&criteria.pixels.len() - 1).unwrap();

        let iter_pixels = criteria.pixels.clone();
        for (pos, pixel) in iter_pixels.clone().enumerate() {
            let mut partial_flow = U::zero();
            for other in iter_pixels.clone() {
                let positive = criteria.function.compare(pixel, other);
                let negative = criteria.function.compare(other, pixel);
                partial_flow += positive - negative;
            }
            flow.set(pos, partial_flow / area * criteria.weight);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_linear_criteria() {
        let criteria = Criteria {
            pixels: (0..81).step_by(10).map(|x| x as f64),
            weight: 0.5,
            function: LinearFunction { p: 1.0 },
        };
        let mut res = vec![0f64; 9];
        Vanilla::rank(criteria, res.as_mut());

        assert_eq!(
            res,
            vec![-0.5, -0.375, -0.25, -0.125, -0.0, 0.125, 0.25, 0.375, 0.5]
        );
    }

    #[test]
    fn erosao_sample() {
        let matrix = vec![4.8, 3.4, 3.8, 4.5];
        let criteria = Criteria {
            pixels: matrix.iter().map(|x| *x as f64),
            weight: 2.0,
            function: LinearFunction { p: 5.0 },
        };
        let mut res = vec![0f64; 4];
        Vanilla::rank(criteria, res.as_mut());

        assert_eq!(res, vec![-0.36, 0.38666667, 0.17333333, -0.2]);
    }
}
