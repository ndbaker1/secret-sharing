//! Interface for Shamir Secret Sharing Split and Combine operations
//!
//! reference the GNU ssss implementation: http://point-at-infinity.org/ssss/

use crate::lagrange;

use num::{BigInt, Signed};
use num_bigint::{RandBigInt, Sign};
use rand;

/// Shamir Secret Sharing Scheme Combine Operation
///
/// Takes a set of pairs of points which represent shares
/// to a secret that can be recovered by computing origin
/// f(0) of the curve belonging to the points
pub fn combine(shares: &[(BigInt, BigInt)]) -> BigInt {
    lagrange::interpolate(BigInt::from(0), shares)
}

/// Shamir Secret Sharing Scheme Split Operation
///
/// Takes a series of bytes as the secret, and outputs
/// several pairs of points which hold shares to the secret
pub fn split(secret_bytes: &[u8], t: u8, n: u8) -> Vec<(BigInt, BigInt)> {
    let secret = BigInt::from_bytes_be(Sign::Plus, secret_bytes);

    // Generate coefficients for the curve with
    // the secret at the inital offset
    let mut coefficients = vec![secret];
    coefficients.extend_from_slice(&gen_coefficients::<512>(t - 1));

    // Construct `n` shares from the curve
    (1..=n)
        .into_iter()
        .map(BigInt::from)
        .map(|x| (x.clone(), horner(x, &coefficients)))
        .collect()
}

/// Generates random coefficients for the shamir sharing scheme curve
///
/// * `const R` - number of bits in the random number generation
fn gen_coefficients<const R: u64>(n: u8) -> Vec<BigInt> {
    let mut rng = rand::thread_rng();
    (0..n)
        .into_iter()
        .map(|_| rng.gen_bigint(R))
        .map(|f| &f * f.signum()) // Set the sign to positive
        .collect()
}

/// Horner coefficient polynomial expansion
///
/// Expects to be passed and a slice of coefficients where
/// `coefficients = [a_0, a_1, a_2, .., a_n]`
fn horner(x: BigInt, coefficients: &[BigInt]) -> BigInt {
    coefficients
        .iter()
        .rev()
        .fold(0.into(), |sum, c| sum * &x + c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shamir_e2e_test_32bytes() {
        let secret_bytes = b"32byte message for testing funcs";

        let shares = split(secret_bytes, 3, 5);
        let bytes = combine(&shares);

        assert_eq!(BigInt::from_bytes_be(Sign::Plus, secret_bytes), bytes);
    }

    #[test]
    fn shamir_e2e_test_1byte() {
        let secret_bytes = b"t";

        let shares = split(secret_bytes, 3, 3);
        let bytes = combine(&shares);

        assert_eq!(BigInt::from_bytes_be(Sign::Plus, secret_bytes), bytes);
    }

    #[test]
    fn shamir_e2e_test_repeat_check() {
        let secret_bytes = b"secret text";

        // Detect off by 1 or rounding errors
        for _ in 0..100 {
            let shares = split(secret_bytes, 3, 9);
            let bytes = combine(&[shares[0].clone(), shares[5].clone(), shares[8].clone()]);

            assert_eq!(BigInt::from_bytes_be(Sign::Plus, secret_bytes), bytes);
        }
    }

    #[test]
    fn shamir_bad_shares() {
        let secret_bytes = b"t";

        let shares = split(secret_bytes, 3, 5);
        let bytes = combine(&shares[0..2]);

        assert_ne!(BigInt::from_bytes_be(Sign::Plus, secret_bytes), bytes);
    }
}
