use num::{FromPrimitive, One};
use num_bigint::BigUint;
use num_complex::Complex;

const SMALL_PRIMES: [usize; 128] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 435, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
    547, 557, 561, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647,
    653, 659, 661, 673, 677, 683, 691, 701,
];
pub fn first_n_primes(n: usize) -> Vec<usize> {
    let mut results = [2, 3, 5, 7].to_vec();
    let max_limit = inverse_primecount_estimator_upper(&(n as f64)).round() as usize;
    let start_count = 8;
    for i in start_count..max_limit {
        if miller_rabin_primality(&BigUint::from_usize(i).unwrap()) {
            results.push(i);
        }
        if results.len() >= n {
            return results;
        }
    }
    panic!("Could not find enough primes ({} found, {} needed), despite more than {} primes existing under {}", results.len(), n, n,max_limit);
}

// So the prime count estimator pi(x) is always strictly less than x/ln(x) for x>69 according to a
// stackoverflow theorem. (Starting with SMALL_PRIMES ensures we are always over that threshold)
// However, we have a secondary problem, trying to guess how many numbers we have to guess for an
// upper bound so we can have a finite bound for how many numbers to check. So we want to get a
// number strictly bigger than the inverse of x/ln(x). Using algebra, we can create our inverse g(x)
// as g(x)=x*ln(g(x))
// So given an nth order approximation
// g_n(x)=x*ln(g_{n-1}(x))
// if we start with a g_0 strictly greater then g, for example g_0 = x^2. (I am 60% sure that this
// will make sure the approximations will be universally bigger, but I am far to lazy to throw
// together a proof) Then we get the approximations
// g_1 = x * ln(x^2) = 2x*ln(x)
// g_2 = x* ln(2x * ln(x))
// g_3 = x * ln( x * ln( 2x * ln ( x)))
// going with the second approximation, since this whole investigation is an optimization with
// absolutely no performance benefit and is deeply unimportant.

fn inverse_primecount_estimator_upper(x: &f64) -> f64 {
    x * (x * 2.0 * x.ln()).ln()
}

// fn numerical_differential_inverse(
//     f: impl Fn(&f64) -> f64,
//     fprime: impl Fn(&f64) -> f64,
// ) -> impl Fn(&f64) -> f64 {
//     move |x: &f64| -> f64 {
//         let newton_func = |z| f(z) - x;
//         let inverse = newtons_method_rootfind_primative(newton_func, fprime, x);
//         inverse
//     }
// }
// fn newtons_method_rootfind_primative(
//     f: impl Fn(&f64) -> f64,
//     fprime: impl Fn(&f64) -> f64,
//     guess: &f64,
// ) -> f64 {
//     let mut iterator_guess = guess - f(guess) / fprime(guess);
//     const MAX_ITER: u32 = 5;
//     for _ in 0..MAX_ITER {
//         iterator_guess = iterator_guess - f(&iterator_guess) / fprime(&iterator_guess);
//     }
//     return iterator_guess;
// }
//
// fn hallys_method_rootfind_primative(
//     f: impl Fn(&f64) -> f64,
//     fprime: impl Fn(&f64) -> f64,
//     fprimeprime: impl Fn(&f64) -> f64,
//     guess: f64,
// ) -> f64 {
//     let mut iterator_guess = guess;
//     const MAX_ITER: u32 = 5;
//     for _ in 0..MAX_ITER {
//         iterator_guess = iterator_guess
//             - 2.0 * (f(&iterator_guess) * fprime(&iterator_guess))
//                 / (2.0 * fprime(&iterator_guess).powf(2.0)
//                     - f(&iterator_guess) * fprimeprime(&iterator_guess));
//     }
//     return iterator_guess;
// }

pub fn miller_rabin_primality(num: &BigUint) -> bool {
    if num % 2 as usize == BigUint::ZERO {
        return false;
    }
    fn even_exp_factorize(num: &BigUint) -> (BigUint, u64) {
        let exp = num.trailing_zeros();
        if let Some(exp) = exp {
            let odd = num << exp;
            return (odd, exp);
        }
        panic!("Somehow you encountered unreachable code by passing zero into the primality function, dispite a check for zero existing.")
    }

    fn miller_rabin_iteration(
        witness: u32,
        z: &BigUint,
        z_odd: &BigUint,
        z_exp: u64,
        num: &BigUint,
    ) -> bool {
        // Choose a witness between 2 and n-2
        let mut x = BigUint::from(witness).modpow(z_odd, num);
        if x.is_one() {
            return true;
        }
        for _ in 0..z_exp {
            if &x == z {
                return true;
            }
            x = x.pow(2) % num;
            if x.is_one() {
                return false;
            }
        }
        false
    }
    let z = num - (1 as u32);
    let (z_odd, z_exp) = even_exp_factorize(&z);

    for witness in [2, 3, 5, 7, 61] {
        if !miller_rabin_iteration(witness, &z, &z_odd, z_exp, num) {
            return false;
        }
    }
    true
}

fn eitau_real(x: &f32) -> Complex<f32> {
    let z = x * std::f32::consts::TAU;
    Complex::new(z.cos(), z.sin())
}

fn small_prime_factorize(num: &BigUint) -> Option<[BigUint; 2]> {
    // let largest_small_prime = small_primes[small_primes.len() - 1];
    for prime in SMALL_PRIMES {
        if num % prime == BigUint::ZERO {
            return Some([num.clone() / prime, BigUint::from_usize(prime).unwrap()]);
        }
    }
    None
}

fn generate_smooth_vector(num: &BigUint, list: &Vec<usize>) -> Option<Vec<u8>> {
    let mut remainder = num.clone();
    let mut result_exponents: Vec<u8> = vec![0; list.len()];
    fn trial_division_singleton_index(num: &BigUint, list: &Vec<usize>) -> Option<usize> {
        // Chance this for an index.
        for (index, trial) in list.iter().enumerate() {
            if num % trial == BigUint::ZERO {
                return Some(index);
            }
        }
        None
    }
    while remainder > BigUint::one() {
        match trial_division_singleton_index(&remainder, list) {
            None => return None,
            Some(index) => {
                result_exponents[index] += 1;
                remainder = remainder / list[index];
            }
        }
    }
    return Some(result_exponents);
}

fn quadratic_number_sieve_factor(num: &BigUint) -> Option<[BigUint; 2]> {
    const SMOOTHNESS_DIMENSION: usize = 32;
    let primes: Vec<usize> = SMALL_PRIMES.to_vec();
    let mut results: Vec<(BigUint, Vec<u8>)> = Vec::new();
    let mut test = num.sqrt() + 1 as usize;
    let mut results_found: usize = 0;
    while results_found < SMOOTHNESS_DIMENSION + 1 {
        let test_squared = test.pow(2) % num;
        if let Some(divisors) = generate_smooth_vector(&test_squared, &primes) {
            results.push((test.clone(), divisors));
            results_found += 1;
        }
        test += 1 as usize;
    }

    // So the problem has now been reduced to: given a set of integers, find a subset whose product is a square. By the [[fundamental theorem of arithmetic]], any positive integer can be written uniquely as a product of [[prime power]]s. We do this in a vector format; for example, the prime-power factorization of 504 is 2<sup>3</sup>3<sup>2</sup>5<sup>0</sup>7<sup>1</sup>, it is therefore represented by the exponent vector (3,2,0,1). Multiplying two integers then corresponds to adding their exponent vectors. A number is a square when its exponent vector is even in every coordinate. For example, the vectors (3,2,0,1) + (1,0,0,1) = (4,2,0,2), so (504)(14) is a square. Searching for a square requires knowledge only of the [[parity (mathematics)|parity]] of the numbers in the vectors, so it is sufficient to compute these vectors mod 2: (1,0,0,1) + (1,0,0,1) = (0,0,0,0).
    // So given a set of (0,1)-vectors, we need to find a subset which adds to the [[zero vector]] mod 2.
    //
    // This is a [[linear algebra]] problem since the [[ring (mathematics)|ring]] <math>\mathbb{Z}/2\mathbb{Z}</math> can be regarded as the [[Galois field]] of order 2, that is we can divide by all non-zero numbers (there is only one, namely 1) when calculating modulo 2.
    // It is a [[Rank–nullity theorem|theorem of linear algebra]] that with more vectors than each vector has entries, a [[linear dependency]] always exists. It can be found by [[Gaussian elimination]].
    // However, simply squaring many random numbers mod ''n'' produces a very large number of different [[prime number|prime]] factors, and thus very long vectors and a very large matrix. The trick is to look specifically  for numbers ''a'' such that ''a''<sup>2</sup> mod ''n'' has only small prime factors (they are [[smooth number]]s). They are harder to find, but using only smooth numbers keeps the vectors and matrices smaller and more tractable. The quadratic sieve searches for smooth numbers using a technique called [[sieve theory|sieving]], discussed later, from which the algorithm takes its name.
    //
    // To summarize, the basic quadratic sieve algorithm has these main steps:
    //
    //     Choose a smoothness bound B. The number π(B), denoting the number of prime numbers less than B, will control both the length of the vectors and the number of vectors needed.
    //     Use sieving to locate π(B) + 1 numbers a_i such that b_i = (a_i^2 mod n) is B-smooth.
    //     Factor the b_i and generate exponent vectors mod 2 for each one.
    //     Use linear algebra to find a subset of these vectors which add to the zero vector. Multiply the corresponding ai together and give the result mod n the name a; similarly, multiply the bi together which yields a B-smooth square b2.
    //     We are now left with the equality a^2 = b^2 mod n from which we get two square roots of (a^2 mod n), one by taking the square root in the integers of b^2 namely b, and the other the a computed in step 4.
    //     We now have the desired identity: ( a + b ) ( a − b ) ≡ 0 ( mod n ) {\displaystyle (a+b)(a-b)\equiv 0{\pmod {n}}}. Compute the GCD of n with the difference (or sum) of a and b. This produces a factor, although it may be a trivial factor (n or 1). If the factor is trivial, try again with a different linear dependency or different a.
    None
}

fn factor_number(num: &BigUint) -> Result<Vec<BigUint>, String> {
    let mut main_num = num.clone();
    let mut results: Vec<BigUint> = vec![];
    while let Some([big, small]) = small_prime_factorize(&main_num) {
        results.push(small);
        main_num = big;
    }
    fn factor_number_recursive(num: BigUint) -> Result<Vec<BigUint>, String> {
        if miller_rabin_primality(&num) {
            return Ok(vec![num]);
        }
        match quadratic_number_sieve_factor(&num) {
            Some([factor1, factor2]) => {
                let mut factorized_1 = factor_number_recursive(factor1)?;
                let mut factorized_2 = factor_number_recursive(factor2)?;
                factorized_1.append(&mut factorized_2);
                Ok(factorized_1)
            }
            None => Err("Error factoring num with quadratic number sieve.".to_string()),
        }
    }
    let mut quadratic_results = factor_number_recursive(main_num)?;
    results.append(&mut quadratic_results);
    results.sort_unstable();
    Ok(results)
}

fn slow_fourier_transform(sequence: &[Complex<f32>]) -> Vec<Complex<f32>> {
    let mut res: Vec<Complex<f32>> = Vec::new();
    for i in 0..sequence.len() {
        let mut sum = Complex::new(0.0, 0.0);
        for j in 0..sequence.len() {
            sum += sequence[j] * eitau_real(&((i * j) as f32 / sequence.len() as f32));
        }
        res.push(sum);
    }
    res
}

// A(x)=a_n x^(n) + a_(n-1) x^(n-1) + ... + a_1 x + a_0
// This polynomial needs to be evaulated at n different values of x
// For two polynomials, then Odd(x)=-Odd(-x), and Even(x)=Even(-x)
// For every polynomial A(x) = B(x^2) + xC(x^2)
// And therefore A(-x) = B(x^2) - xC(x^2)
// So for any polynomial, we can compute A(-x) and A(x) at the same time.
// Ideally we want to do the same for B(x), and C(x), but in order to get a good set of pairs, we
// evaluate ut at zeta, where zeta^(2^n)-1=0
// then for B(x) and C(x) have to be evaulated at zeta^(2^(n-1)). Half the valuations of
// previously.
// What I dont understand is how to generalize this outside powers of 2.
// One spoiler free wikipedia glance and I got that it only works for composites, and cant break
// down primes. Thats helpful. We can see that it can split off factors of 2. How about 3?
//
// Split A(x) = B(x^3) + xC(x^3) + x^2D(x^3)
// Then let w^3=1. Then we should have
// A(x) = B(x^3) + xC(x^3) + x^2D(x^3)
// A(wx) = B(w^3x^3) + wxC(w^3x^3) + w^2x^2D(w^3x^3) = B(x^3) + wxC(x^3) + w^2x^2D(x^3)
// A(w^2x) = B(w^6x^3) + w^2xC(w^6x^3) + w^4x^2D(w^6x^3) = B(x^3) + w^2xC(x^3) + wx^2D(x^3)
// also
// more generally any polynomial P(x) with order pq, can be split
// P(x) = sum [x^i*P_i(x^p) for i in 0..p]
// and for any zeta^(p)-1=0
// P(zeta^k) = sum [x^i zeta^ik * P_i(x^p) for i in 0..p]
// So how much does it cost to compute the whole DFT for zeta^pq -1 =0, how much does
// computing P(zeta^k) for k=0..pq cost? (assuming p and q prime so I dont have to think about
// recursion)
//
// All of the P_i(x^p) have order q, and you need to compute a slow DFT for each of them, and there
// are p of them. So the total cost for the compute step is O(pq^2), then for the entire combination
// process needs to be run q times for each result DFT generated by the sub polynomials. And each
// process involves doing p multiplications p times. So the cost of the combo process is O(p^2q).
// Giving us a total cost of O(p q^2 + p^2 q), compared to the slow DFT of O(p^2 q^2). I understand
// why this is only presented as a solution for powers of 2 lol. Although for pq=n and p roughly
// the same size as q. This does go from (n^2) to (n* sqrt(n))
fn fast_fourier_transform_recursive(
    sequence: &[Complex<f32>],
    length_factorized: &[usize],
) -> Vec<Complex<f32>> {
    let length = length_factorized.iter().fold(1, |a, b| a * b);
    if sequence.len() != length {
        panic!("Lengths do not match");
    }
    if length_factorized.len() == 1 {
        return slow_fourier_transform(sequence);
    }
    let main_prime = length_factorized[0];
    let leftovers = sequence.len() / main_prime;
    let mut smaller_polynomials: Vec<Vec<Complex<f32>>> = vec![vec![]; main_prime];
    for i in 0..main_prime {
        for j in 0..leftovers {
            smaller_polynomials[i].push(sequence[i + j * main_prime]);
        }
    }
    let leftover_factorized = &(length_factorized[1..].to_vec());
    let small_fourier = |sub_sequence: &Vec<Complex<f32>>| -> Vec<Complex<f32>> {
        fast_fourier_transform_recursive(sub_sequence, leftover_factorized)
    };
    let sub_fourier_results: Vec<Vec<Complex<f32>>> =
        smaller_polynomials.iter().map(small_fourier).collect();
    let mut fourier_return: Vec<Complex<f32>> = vec![Complex::ZERO; length];
    for j in 0..leftovers {
        for i in 0..main_prime {
            let mut sum = Complex::ZERO;
            for k in 0..main_prime {
                sum +=
                    sub_fourier_results[k][j] * eitau_real(&((i * k) as f32 / main_prime as f32));
            }
            fourier_return[j * main_prime + i] = sum;
        }
    }
    fourier_return
}
