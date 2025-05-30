mod categories;
mod datastructures;
mod math;
mod physics;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    generate_test_julia_fractal: u64,
}

fn main() {
    let args = Args::parse();
}
