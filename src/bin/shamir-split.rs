use clap::Parser;
use secret_sharing::shamir;

/// Program to split secrets using Shamir-Shared Secret Scheme
#[derive(Parser)]
#[clap(author, version)]
struct ShamirSplitArgs {
    /// Number of shares to distribute for the secret
    #[clap(short = 'n', long)]
    share_count: u8,

    /// Threshold secret share count until secret is recoverable
    #[clap(short = 'k', long)]
    threshold: u8,

    /// Secret to split using Shamir Secret Sharing Scheme
    #[clap(short, long)]
    secret: String,
}

fn main() {
    let args = ShamirSplitArgs::parse();

    println!(
        "Generating shares using a ({},{}) scheme with dynamic security level.",
        args.threshold, args.share_count
    );
    print!("Enter the secret, at most 128 ASCII characters: ");

    let mut line = String::new();
    let secret = std::io::stdin().read_line(&mut line).unwrap();

    // Using a 184 bit security level.
    // 1-1c41ef496eccfbeba439714085df8437236298da8dd824
    // 2-fbc74a03a50e14ab406c225afb5f45c40ae11976d2b665
    // 3-fa1c3a9c6df8af0779c36de6c33f6e36e989d0e0b91309
    // 4-468de7d6eb36674c9cf008c8e8fc8c566537ad6301eb9e
    // 5-4756974923c0dce0a55f4774d09ca7a4865f64f56a4ee0

    println!(
        "{:#?}",
        shamir::split(args.secret.as_bytes(), args.threshold, args.share_count)
    );
}
