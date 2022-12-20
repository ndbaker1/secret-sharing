//! Lagrange Interpolation for computing a polynomial of degree k from k distinct points

use num::{BigInt, One};

/// Using the Lagrange Polynomial Interpolation method,
/// compute f(x) given an adequate number of points
/// from the function, where `num_points = degree(p(x)) + 1`
///
/// Writes the interpolation in a way that saves all division
/// for the very last step to avoid any sneaky integer division
///
/// # Example
///
/// say you have following lagrange polynomial equation:
/// ```math
/// L(x) = ( y_1 * (x - x_2) / (x_1 - x_2) ) + ( y_2 * (x - x_1) / (x_2 - x_1) )
/// ```
///
/// Cross-multiply along the way to end up with:
/// ```math
/// L(x) = ( y_1 * (x - x_2) * (x_2 - x_1) + y_2 * (x - x_1) * (x_1 - x_2) ) / ( (x_1 - x_2) * (x_2 - x_1) )
/// ```
pub fn interpolate(x: BigInt, points: &[(BigInt, BigInt)]) -> BigInt {
    let mut basis_numerators = vec![BigInt::one(); points.len()];
    let mut basis_denominator = BigInt::one();

    for (i, (x_inner, y)) in points.iter().enumerate() {
        basis_numerators[i] *= y;

        for (x_outer, _) in points.iter().filter(|(x_outer, _)| x_outer != x_inner) {
            let denominator = x_inner - x_outer;
            let numerator = &x - x_outer;

            // split the denominator among all basis numerators
            for basis_numerator in basis_numerators.iter_mut() {
                *basis_numerator *= &denominator;
            }

            basis_numerators[i] *= &numerator;
            basis_denominator *= &denominator;

            // remove our own denominator
            basis_numerators[i] /= &denominator;
        }
    }

    basis_numerators.into_iter().sum::<BigInt>() / basis_denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate_test() {
        // f(x) = x ; f(0) = 0
        assert_eq!(
            BigInt::from(0),
            interpolate(0.into(), &[(1.into(), 1.into()), (2.into(), 2.into())])
        );
        // f(x) = x^2; f(4) = 16
        assert_eq!(
            BigInt::from(16),
            interpolate(
                4.into(),
                &[
                    (1.into(), 1.into()),
                    (2.into(), 4.into()),
                    (5.into(), 25.into())
                ]
            )
        );
        // f(x) = (x + 1)^2; f(4) = 25
        assert_eq!(
            BigInt::from(25),
            interpolate(
                4.into(),
                &[
                    (1.into(), 4.into()),
                    (2.into(), 9.into()),
                    (5.into(), 36.into())
                ]
            )
        );
        // f(x) = x^2 + 1; f(4) = 17
        assert_eq!(
            BigInt::from(17),
            interpolate(
                4.into(),
                &[
                    (1.into(), 2.into()),
                    (2.into(), 5.into()),
                    (5.into(), 26.into())
                ]
            )
        );
    }
}
