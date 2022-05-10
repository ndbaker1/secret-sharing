//! Interface for Shamir Secret Sharing split and coming operations
//!
//! goal to also be compatible with GNU ssss implementation: http://point-at-infinity.org/ssss/
//!

use crate::lagrange;

pub fn combine(shares: &[String]) -> Vec<u8> {
    let keyed_shares: Vec<_> = shares
        .into_iter()
        .map(|share| parse_share_str(&share))
        .collect();

    println!("{keyed_shares:#?}");

    let f_at_0 = lagrange::interpolate(0f64, &keyed_shares);

    println!("{f_at_0}");

    (f_at_0 as u64).to_be_bytes().to_vec()
}

pub fn split(secret_bytes: &[u8], k: u64, n: u64) -> Result<Vec<String>, ()> {
    let mut rng = rand::thread_rng();

    // let secret_to_int = 0;
    // let mut coefficients = vec![secret_int];
    // coefficients.extend((0..k).map(|_| /* rand */).collect());

    let shares = gen_coefficients(n).iter().map(|x| {
        // go from highest to lowest degree to compute using horner's polynomial expansion
        let y = 0;

        (x, y)
    });

    Ok(Vec::new())
}

fn parse_share_str(share: &str) -> (f64, f64) {
    let split: Vec<_> = share.split('-').collect();
    let (x, hex) = (split[0], split[1]);
    (x.parse().unwrap(), hex_to_f64(hex))
}

fn gen_coefficients(n: u64 /* , prime: f64 */) -> Vec<u64> {
    // TODO
    Vec::new()
}

fn hex_to_f64(hex_str: &str) -> f64 {
    hex_str
        .bytes()
        .fold(0f64, |acc, next| acc * 8f64 + f64::from(next))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shamir_recover_test() {
        let secret_bytes = b"32byte message for testing funcs";
        let shares = split(secret_bytes, 3, 5).unwrap();
        println!("generated secrets:\n{shares:#?}");
        let bytes = combine(&shares);
        assert_eq!(secret_bytes, bytes.as_slice());
    }
}
