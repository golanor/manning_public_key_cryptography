pub mod rsa {
    use crate::primality_testing::primal_testing;
    use crate::{fast_exp_mod, gcd, get_u64, lcm};
    use rand::{self, Rng};

    pub fn totient(p: u64, q: u64) -> u64 {
        lcm(p - 1, q - 1)
    }

    pub fn random_exponent(λ_n: u64) -> u64 {
        let mut prng = rand::thread_rng();
        loop {
            let e = prng.gen_range(3..λ_n);
            if gcd(e, λ_n) == 1 {
                return e;
            }
        }
    }

    pub fn inverse_mod(a: u64, p: u64) -> Option<u64> {
        let mut t = 0_i64;
        let mut r = p;
        let mut new_t = 1_i64;
        let mut new_r = a;
        while new_r != 0 {
            let q = r / new_r;
            (t, new_t) = (new_t, t - (q as i64) * new_t);
            (r, new_r) = (new_r, r - q * new_r);
        }
        if r > 1 {
            return None;
        }
        if t < 0 {
            t = t + (p as i64);
        }
        Some(t as u64)
    }

    pub fn main() {
        let p = primal_testing::find_prime(1_000, 10_000, 20);
        let q = primal_testing::find_prime(1_000, 10_000, 20);
        let n = p * q;
        let carmichael = totient(p, q);
        let e = random_exponent(carmichael);
        let d = inverse_mod(e, carmichael).unwrap();
        println!("***** PUBLIC *****");
        println!("Public Key Modulus: {}", n);
        println!("Public Key Exponent: {e}");
        println!("");
        println!("***** PRIVATE *****");
        println!("Primes: {p}, {q}");
        println!("λ(n): {carmichael}");
        println!("d: {d}");
        loop {
            let m = get_u64("Enter a message between 1 and the public_key: ");
            let encrypted = fast_exp_mod(m, e, n);
            let decrypted = fast_exp_mod(encrypted, d, n);
            assert_eq!(m, decrypted);
        }
    }
}
