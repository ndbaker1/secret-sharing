//! Interface for Shamir Secret Sharing split and coming operations
//!
//! goal to also be compatible with GNU ssss implementation: http://point-at-infinity.org/ssss/

use crate::lagrange;

pub fn combine(shares: &[(lagrange::NBase, lagrange::NBase)]) -> Vec<u8> {
    let f_at_0 = lagrange::interpolate(0, &shares);

    f_at_0.to_be_bytes().to_vec()
}

pub fn split(
    secret_bytes: &[u8],
    k: u8,
    n: u8,
) -> Result<Vec<(lagrange::NBase, lagrange::NBase)>, ()> {
    //let mut rng = rand::thread_rng();

    // let secret_to_int = 0;
    // let mut coefficients = vec![secret_to_int];
    // coefficients.extend((0..k).map(|_| 0 /* rand */).collect());

    //let shares = gen_coefficients(n).iter().map(|x| {
    //    // go from highest to lowest degree to compute using horner's polynomial expansion
    //    let y = 0;

    //    (x, y)
    //});

    Ok(Vec::new())
}

/// Generates random coefficients for the shamir sharing scheme curve
fn gen_coefficients(n: u8 /* , prime: f64 */) -> Vec<u64> {
    // TODO
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shamir_e2e_test() {
        let secret_bytes = b"32byte message for testing funcs";

        let shares = split(secret_bytes, 3, 5).unwrap();
        let bytes = combine(&shares);

        assert_eq!(secret_bytes, bytes.as_slice());
    }

    #[test]
    fn shamir_recover_test() {
        // utilize the function f(x) = x^2 + 5
        let points = [(1u8, 6u8), (2u8, 9u8), (4u8, 21u8)];
        // map the points into shared key strings
        let shares: Vec<String> = points
            .iter()
            .map(|(x, y)| {
                let encoded_y = String::from_utf8(y.to_be_bytes().to_vec()).unwrap();
                format!("{}-{}", x, encoded_y)
            })
            .collect();

        // assert_eq!(Some(&5), combine(&shares).last());
    }
}
