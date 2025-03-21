mod colors;

mod fast_fourier_transform;
pub use fast_fourier_transform::slow_fourier_transform;

mod fractals;
pub use fractals::generate_julia_image;

mod primes;
pub use primes::{WitnessSet, miller_rabin_primality};
