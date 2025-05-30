// // So the prime count estimator pi(x) is always strictly less than x/ln(x) for x>69 according to a
// // stackoverflow theorem. (Starting with SMALL_PRIMES ensures we are always over that threshold)
// // However, we have a secondary problem, trying to guess how many numbers we have to guess for an
// // upper bound so we can have a finite bound for how many numbers to check. So we want to get a
// // number strictly bigger than the inverse of x/ln(x). Using algebra, we can create our inverse g(x)
// // as g(x)=x*ln(g(x))
// // So given an nth order approximation
// // g_n(x)=x*ln(g_{n-1}(x))
// // if we start with a g_0 strictly greater then g, for example g_0 = x^2. (I am 60% sure that this
// // will make sure the approximations will be universally bigger, but I am far to lazy to throw
// // together a proof) Then we get the approximations
// // g_1 = x * ln(x^2) = 2x*ln(x)
// // g_2 = x* ln(2x * ln(x))
// // g_3 = x * ln( x * ln( 2x * ln ( x)))
// // going with the second approximation, since this whole investigation is an optimization with
// // absolutely no performance benefit and is deeply unimportant.
//
// fn inverse_primecount_estimator_upper(x: &f64) -> f64 {
//     x * (x * 2.0 * x.ln()).ln()
// }
//
// // TODO: Since the results with with only 2 as a witness are identical for <2,047. Test that the
// // results under that threshold are identical with a huge witness set. Same for {2,3} being valid
// // for <1,373,653.
// pub fn small_is_prime<N: SmallNatural>(n: &N) -> bool {
//     miller_rabin_primality(&n.to_big_uint(), &WitnessSet::Only23.generate_witness())
// }
//
// // This enum should store values of type <const N: usize>[u32; N]. But I dont know how to do that.
// // Ideally everything should have a declared result at compile time.
// #[derive(Default, Debug)]
// pub enum WitnessSet {
//     #[default]
//     Only2,
//     Only23,
//     Only2761,
//     Random(usize),
// }
//
// impl WitnessSet {
//     pub fn generate_witness(&self) -> Vec<u32> {
//         match self {
//             WitnessSet::Only2 => vec![2],
//             WitnessSet::Only23 => vec![2, 3],
//             WitnessSet::Only2761 => vec![2, 7, 61],
//             WitnessSet::Random(n) => {
//                 let mut rng = rand::thread_rng();
//                 let mut results = Vec::with_capacity(*n);
//                 while results.len() < *n {
//                     results.push(rng.gen_range(11..usize::MAX) as u32);
//                 }
//                 results
//             }
//         }
//     }
// }
//
// pub fn miller_rabin_primality(num: &BigUint, witnesses: &Vec<u32>) -> bool {
//     if num % 2 as usize == BigUint::ZERO {
//         return false;
//     }
//     fn even_exp_factorize(num: &BigUint) -> (BigUint, u64) {
//         let exp = num.trailing_zeros();
//         if let Some(exp) = exp {
//             let odd = num << exp;
//             return (odd, exp);
//         }
//         panic!("Somehow you encountered unreachable code by passing zero into the primality function, dispite a check for zero existing.")
//     }
//
//     fn miller_rabin_iteration(
//         witness: &u32,
//         z: &BigUint,
//         z_odd: &BigUint,
//         z_exp: u64,
//         num: &BigUint,
//     ) -> bool {
//         // Choose a witness between 2 and n-2
//         let mut x = BigUint::from(witness.clone()).modpow(z_odd, num);
//         if x.is_one() {
//             return true;
//         }
//         for _ in 0..z_exp {
//             if &x == z {
//                 return true;
//             }
//             x = x.pow(2) % num;
//             if x.is_one() {
//                 return false;
//             }
//         }
//         false
//     }
//     let z = num - (1 as u32);
//     let (z_odd, z_exp) = even_exp_factorize(&z);
//
//     for witness in witnesses {
//         if num % witness == BigUint::ZERO {
//             if num > &BigUint::from(witness.clone()) {
//                 return false;
//             }
//             return true;
//         }
//         if !miller_rabin_iteration(witness, &z, &z_odd, z_exp, num) {
//             return false;
//         }
//     }
//     true
// }
//
// fn small_prime_factorize(num: &BigUint) -> Option<[BigUint; 2]> {
//     // let largest_small_prime = small_primes[small_primes.len() - 1];
//     for prime in SMALL_PRIMES {
//         if num % prime == BigUint::ZERO {
//             return Some([num.clone() / prime, BigUint::from_usize(prime).unwrap()]);
//         }
//     }
//     None
// }
//
// fn generate_smooth_vector(num: &BigUint, list: &Vec<usize>) -> Option<Vec<u8>> {
//     let mut remainder = num.clone();
//     let mut result_exponents: Vec<u8> = vec![0; list.len()];
//     fn trial_division_singleton_index(num: &BigUint, list: &Vec<usize>) -> Option<usize> {
//         // Chance this for an index.
//         for (index, trial) in list.iter().enumerate() {
//             if num % trial == BigUint::ZERO {
//                 return Some(index);
//             }
//         }
//         None
//     }
//     while remainder > BigUint::one() {
//         match trial_division_singleton_index(&remainder, list) {
//             None => return None,
//             Some(index) => {
//                 result_exponents[index] += 1;
//                 remainder = remainder / list[index];
//             }
//         }
//     }
//     return Some(result_exponents);
// }
//
// fn quadratic_number_sieve_factor(num: &BigUint) -> Option<[BigUint; 2]> {
//     const SMOOTHNESS_DIMENSION: usize = 32;
//     let primes: Vec<usize> = SMALL_PRIMES.to_vec();
//     let mut results: Vec<(BigUint, Vec<u8>)> = Vec::new();
//     let mut test = num.sqrt() + 1 as usize;
//     let mut results_found: usize = 0;
//     while results_found < SMOOTHNESS_DIMENSION + 1 {
//         let test_squared = test.pow(2) % num;
//         if let Some(divisors) = generate_smooth_vector(&test_squared, &primes) {
//             results.push((test.clone(), divisors));
//             results_found += 1;
//         }
//         test += 1 as usize;
//     }
//
//     // So the problem has now been reduced to: given a set of integers, find a subset whose product is a square. By the [[fundamental theorem of arithmetic]], any positive integer can be written uniquely as a product of [[prime power]]s. We do this in a vector format; for example, the prime-power factorization of 504 is 2<sup>3</sup>3<sup>2</sup>5<sup>0</sup>7<sup>1</sup>, it is therefore represented by the exponent vector (3,2,0,1). Multiplying two integers then corresponds to adding their exponent vectors. A number is a square when its exponent vector is even in every coordinate. For example, the vectors (3,2,0,1) + (1,0,0,1) = (4,2,0,2), so (504)(14) is a square. Searching for a square requires knowledge only of the [[parity (mathematics)|parity]] of the numbers in the vectors, so it is sufficient to compute these vectors mod 2: (1,0,0,1) + (1,0,0,1) = (0,0,0,0).
//     // So given a set of (0,1)-vectors, we need to find a subset which adds to the [[zero vector]] mod 2.
//     //
//     // This is a [[linear algebra]] problem since the [[ring (mathematics)|ring]] <math>\mathbb{Z}/2\mathbb{Z}</math> can be regarded as the [[Galois field]] of order 2, that is we can divide by all non-zero numbers (there is only one, namely 1) when calculating modulo 2.
//     // It is a [[Rank–nullity theorem|theorem of linear algebra]] that with more vectors than each vector has entries, a [[linear dependency]] always exists. It can be found by [[Gaussian elimination]].
//     // However, simply squaring many random numbers mod ''n'' produces a very large number of different [[prime number|prime]] factors, and thus very long vectors and a very large matrix. The trick is to look specifically  for numbers ''a'' such that ''a''<sup>2</sup> mod ''n'' has only small prime factors (they are [[smooth number]]s). They are harder to find, but using only smooth numbers keeps the vectors and matrices smaller and more tractable. The quadratic sieve searches for smooth numbers using a technique called [[sieve theory|sieving]], discussed later, from which the algorithm takes its name.
//     //
//     // To summarize, the basic quadratic sieve algorithm has these main steps:
//     //
//     //     Choose a smoothness bound B. The number π(B), denoting the number of prime numbers less than B, will control both the length of the vectors and the number of vectors needed.
//     //     Use sieving to locate π(B) + 1 numbers a_i such that b_i = (a_i^2 mod n) is B-smooth.
//     //     Factor the b_i and generate exponent vectors mod 2 for each one.
//     //     Use linear algebra to find a subset of these vectors which add to the zero vector. Multiply the corresponding ai together and give the result mod n the name a; similarly, multiply the bi together which yields a B-smooth square b2.
//     //     We are now left with the equality a^2 = b^2 mod n from which we get two square roots of (a^2 mod n), one by taking the square root in the integers of b^2 namely b, and the other the a computed in step 4.
//     //     We now have the desired identity: ( a + b ) ( a − b ) ≡ 0 ( mod n ) {\displaystyle (a+b)(a-b)\equiv 0{\pmod {n}}}. Compute the GCD of n with the difference (or sum) of a and b. This produces a factor, although it may be a trivial factor (n or 1). If the factor is trivial, try again with a different linear dependency or different a.
//     None
// }
//
// fn factor_number(num: &BigUint) -> Result<Vec<BigUint>, String> {
//     let mut main_num = num.clone();
//     let mut results: Vec<BigUint> = vec![];
//     while let Some([big, small]) = small_prime_factorize(&main_num) {
//         results.push(small);
//         main_num = big;
//     }
//     fn factor_number_recursive(num: BigUint) -> Result<Vec<BigUint>, String> {
//         if miller_rabin_primality(&num, &WitnessSet::default().gen()) {
//             return Ok(vec![num]);
//         }
//         match quadratic_number_sieve_factor(&num) {
//             Some([factor1, factor2]) => {
//                 let mut factorized_1 = factor_number_recursive(factor1)?;
//                 let mut factorized_2 = factor_number_recursive(factor2)?;
//                 factorized_1.append(&mut factorized_2);
//                 Ok(factorized_1)
//             }
//             None => Err("Error factoring num with quadratic number sieve.".to_string()),
//         }
//     }
//     let mut quadratic_results = factor_number_recursive(main_num)?;
//     results.append(&mut quadratic_results);
//     results.sort_unstable();
//     Ok(results)
// }
