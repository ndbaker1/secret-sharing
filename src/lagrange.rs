//! Lagrange Interpolation for computing a polynomial of degree k from k distinct points
//!

/// compute the value of f(x) given the necessary amount of points to interpolate the function
pub fn interpolate(x: f64, f_set: &[(f64, f64)]) -> f64 {
    let lagrange_basis_set = f_set.iter().map(|&(outer, _)| {
        f_set
            .iter()
            // ignore x values that are the same, because they will reduce to 1
            .filter(|&&(inner, _)| inner != outer)
            // compute lagrange basis
            .fold(1f64, |basis, &(inner, _)| basis * -inner / (outer - inner))
    });

    lagrange_basis_set
        .zip(f_set.iter().map(|(_, y)| y))
        .fold(0f64, |acc, (y, basis)| acc + y * basis)
}
