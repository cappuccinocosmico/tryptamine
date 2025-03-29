mod categories;
mod math;
use clap::Parser;
use math::miller_rabin_primality;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    isprime: u64,
}

fn main() {
    let args = Args::parse();

    for test in 0..args.isprime {
        let test_prime = miller_rabin_primality(test.into(), &[2, 3].to_vec());
        match test_prime {
            true => println!("{} is a prime number", test),
            false => println!("{} is not a prime number", test),
        }
    }
}
