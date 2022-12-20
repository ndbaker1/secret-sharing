use std::io::Write;

use clap::Parser;
use num::BigInt;
use secret_sharing::shamir;

/// Program to combine secrets using Shamir-Shared Secret Scheme
#[derive(Parser)]
#[clap(author, version)]
struct ShamirCombineArgs {
    /// Number of shares required to recover the secret
    #[clap(short = 't', long, required = true)]
    threshold: u8,
}

fn main() {
    let args = ShamirCombineArgs::parse();
    println!("Enter {} shares separated by newlines:", args.threshold);

    let shares: Vec<_> = (1..=args.threshold)
        .into_iter()
        .map(|i| {
            print!("Share [{}/{}]: ", i, args.threshold);
            std::io::stdout().flush().unwrap();

            let mut share = String::new();
            std::io::stdin().read_line(&mut share).unwrap();

            share
        })
        .map(|share_str| try_parse_share_str(share_str.trim()))
        .collect();

    println!(
        "Resulting secret: {}",
        String::from_utf8(shamir::combine(&shares).to_bytes_be().1).unwrap(),
    );
}

/// Helper to parse the input hex string as a BigInt
fn try_parse_share_str(share: &str) -> (BigInt, BigInt) {
    let split: Vec<_> = share.split('-').collect();
    let (x, y) = (split[0], split[1]);
    (
        x.parse().unwrap(),
        BigInt::parse_bytes(y.as_bytes(), 16).unwrap(),
    )
}
