use clap::Parser;
use secret_sharing::shamir;

/// Program to combine secrets using Shamir-Shared Secret Scheme
#[derive(Parser)]
#[clap(author, version)]
struct ShamirCombineArgs {
    /// Number of shares to recover the secret
    #[clap(short, long, min_values(1), required = true)]
    shares: Vec<String>,
}

fn main() {
    let args = ShamirCombineArgs::parse();
    println!("{:?}", shamir::combine(&args.shares));
}
