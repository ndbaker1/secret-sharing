//! Lagrange Interpolation for computing a polynomial of degree k from k distinct points
//!

/// compute the value of f(x) given the necessary amount of points to interpolate the function
pub fn interpolate(x: f64, f_set: &[(f64, f64)]) -> f64 {
    let mut lagrange_basis_set = Vec::new();
    for (outer, _) in f_set {
        let mut basis = 1f64;
        // ignore x values that are the same, because they will reduce to 1
        for (inner, _) in f_set.iter().filter(|&(inner, _)| inner != outer) {
            // compute lagrange basis
            let numerator = x - inner;
            let denominator = outer - inner;
            basis = basis * numerator / denominator;
        }
        lagrange_basis_set.push(basis);
    }

    lagrange_basis_set
        .iter()
        .zip(f_set.iter().map(|(_, y)| y))
        .fold(0.into(), |acc, (y, basis)| acc + y * basis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate_test() {
        assert_eq!(
            0f64,
            interpolate(0.into(), &[(1.into(), 1.into()), (2.into(), 2.into())])
        );
    }

    #[test]
    fn interpolate_x_squared() {
        assert_eq!(
            16f64,
            interpolate(
                4.into(),
                &[
                    (1.into(), 1.into()),
                    (2.into(), 4.into()),
                    (3.into(), 9.into()),
                ],
            )
        );
    }

    #[test]
    fn interpolate_x_plus_1_squared() {
        assert_eq!(
            25f64,
            interpolate(
                4.into(),
                &[
                    (1.into(), 4.into()),
                    (2.into(), 9.into()),
                    (3.into(), 16.into()),
                ],
            )
        );
    }

    #[test]
    fn interpolate_x_squared_plus_1() {
        assert_eq!(
            17f64,
            interpolate(
                4.into(),
                &[
                    (1.into(), 2.into()),
                    (2.into(), 5.into()),
                    (3.into(), 10.into()),
                ],
            )
        );
    }
}
