//! Lagrange Interpolation for computing a polynomial of degree k from k distinct points

pub(crate) type NBase = i128;

const GF: NBase = 288;

/// Using the Lagrange Polynomial Interpolation method,
/// compute f(x) given an adequate number of points
/// from the function, where `num_points = degree(p(x)) + 1`
pub fn interpolate(x: NBase, points: &[(NBase, NBase)]) -> NBase {
    points
        .iter()
        // Computing Lagrange Basis Polynomials for each distinct point
        .map(|(x_inner, y)| {
            let (numerator, denominator) = points
                .iter()
                // ignore similar x's to avoid division-by-zero
                .filter(|(x_outer, _)| x_outer != x_inner)
                .fold((1, 1), |frac, (x_outer, _)| {
                    let numerator_step = x - x_outer;
                    let denominator_step = x_inner - x_outer;

                    (frac.0 * numerator_step, frac.1 * denominator_step)
                });

            (numerator / denominator, y)
        })
        // Horner's expansion of polynomial coefficients using fold operation
        .fold(0, |acc, (basis, y)| {
            let sum = acc + y * basis;
            sum.rem_euclid(GF)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate_test() {
        // f(x) = x ; f(0) = 0
        assert_eq!(0, interpolate(0, &[(1, 1), (2, 2)]));
        // f(x) = x^2; f(4) = 16
        assert_eq!(16, interpolate(4, &[(1, 1), (2, 4), (3, 9)]));
        // f(x) = (x + 1)^2; f(4) = 25
        assert_eq!(25, interpolate(4, &[(1, 4), (2, 9), (3, 16)]));
        // f(x) = x^2 + 1; f(4) = 17
        assert_eq!(17, interpolate(4, &[(1, 2), (2, 5), (3, 10)]));
    }
}
