use clap::Parser;

/// Program to split secrets using Shamir-Shared Secret Scheme
#[derive(Parser)]
#[clap(author, version)]
struct ShamirSplitArgs {
    /// Number of shares to distribute for the secret
    #[clap(short = 'n', long)]
    share_count: u64,

    /// Threshold secret share count until secret is recoverable
    #[clap(short = 'k', long)]
    threshold: u64,

    /// Secret to split using Shamir Secret Sharing Scheme
    #[clap(short, long)]
    secret: String,
}

fn main() {
    let args = ShamirSplitArgs::parse();
}
