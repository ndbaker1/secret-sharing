use clap::Parser;
use secret_sharing::{lagrange, shamir};

/// Program to combine secrets using Shamir-Shared Secret Scheme
#[derive(Parser)]
#[clap(author, version)]
struct ShamirCombineArgs {
    /// Number of shares to recover the secret
    #[clap(short = 't', long, required = true)]
    threshold: u8,
}

fn main() {
    let args = ShamirCombineArgs::parse();
    println!("{:?} shares separated by newlines:", args.threshold);

    let shares: Vec<_> = (1..=args.threshold)
        .into_iter()
        .map(|i| {
            print!("[{}/{}]: ", i, args.threshold);
            let mut buffer = String::new();
            let share = std::io::stdin().read_line(&mut buffer).unwrap();
            buffer
        })
        .map(|share_str| try_parse_share_str(&share_str))
        .collect();

    println!("Resulting secret: {:?}", shamir::combine(&shares));
}

fn try_parse_share_str(share: &str) -> (i128, i128) {
    let split: Vec<_> = share.split('-').collect();
    let (x, hex) = (split[0], split[1]);
    (x.parse().unwrap(), parse_hex(hex))
}

fn parse_hex<T>(hex_str: &str) -> T
where
    T: From<u8> + std::ops::Mul<T, Output = T> + std::ops::Add<Output = T>,
{
    hex_str
        .bytes()
        .fold(0.into(), |acc, next| acc * 8.into() + T::from(next))
}
