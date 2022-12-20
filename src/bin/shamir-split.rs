use std::io::Write;

use clap::Parser;
use secret_sharing::shamir;

/// Program to split secrets using Shamir-Shared Secret Scheme
#[derive(Parser)]
#[clap(author, version)]
struct ShamirSplitArgs {
    /// Number of shares to distribute for the secret
    #[clap(short = 'n', long, required = true)]
    share_count: u8,

    /// Threshold secret share count until secret is recoverable
    #[clap(short = 't', long, required = true)]
    threshold: u8,
}

fn main() {
    let args = ShamirSplitArgs::parse();

    println!(
        "Generating shares using a ({},{}) scheme with dynamic security level.",
        args.threshold, args.share_count
    );
    print!("Enter the secret: ");
    std::io::stdout().flush().unwrap();

    let mut secret = String::new();
    std::io::stdin().read_line(&mut secret).unwrap();

    for (x, y) in shamir::split(secret.trim().as_bytes(), args.threshold, args.share_count) {
        println!("{}-{:x}", x, y);
    }
}
