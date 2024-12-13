use num::One;
use num_bigint::BigUint;
fn miller_rabin_primality(num: &BigUint) -> bool {
    let small_primes: Vec<u32> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    // let largest_small_prime = small_primes[small_primes.len() - 1];
    for prime in small_primes {
        if num % prime == BigUint::ZERO {
            return false;
        }
    }
    fn even_exp_factorize(num: &BigUint) -> (BigUint, u64) {
        let exp = num.trailing_zeros();
        if let Some(exp) = exp {
            let odd = num << exp;
            return (odd, exp);
        }
        panic!("num is zero");
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

    for witness in [2, 7, 61] {
        if !miller_rabin_iteration(witness, &z, &z_odd, z_exp, num) {
            return false;
        }
    }
    true
}
