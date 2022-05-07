//! Interface for Shamir Secret Sharing split and coming operations
//!
//! goal to also be compatible with GNU ssss implementation: http://point-at-infinity.org/ssss/
//!

pub fn combine(shares: &[String]) -> Vec<u8> {
    let keyed_shares: Vec<_> = shares
        .into_iter()
        .map(|share| parse_share_str(&share))
        .collect();

    Vec::new() // TODO
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

fn parse_share_str(share: &str) -> (u64, Vec<u8>) {
    let (x, hex) = share.split_at(share.find('-').unwrap());
    // TODO
    (0, Vec::new())
}

fn gen_coefficients(n: u64 /* , prime: f64 */) -> Vec<u64> {
    // TODO
    Vec::new()
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
