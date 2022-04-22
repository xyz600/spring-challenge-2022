// Note: this requires the `derive` feature

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(long)]
    exec1: String,

    /// Number of times to greet
    #[clap(long)]
    exec2: String,
}

fn main() {
    let args = Args::parse();
    eprintln!("battle '{}' vs '{}'", args.exec1, args.exec2);
}
