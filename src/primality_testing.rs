pub mod primal_testing {
    use crate::{fast_exp_mod, get_u64};
    use rand::{self, Rng};
    const NUM_TESTS: u64 = 20;

    pub fn is_probably_prime(p: u64, k: u64) -> bool {
        let mut prng = rand::thread_rng();
        for _ in 0..k {
            let n = prng.gen_range(1..p);
            let mod_exp = fast_exp_mod(n, p - 1, p);
            if mod_exp != 1 {
                return false;
            }
        }
        true
    }

    pub fn find_prime(min: u64, max: u64, k: u64) -> u64 {
        let mut prng = rand::thread_rng();
        loop {
            let p = prng.gen_range(min..max);
            if p % 2 == 0 {
                continue;
            }
            if is_probably_prime(p, k) {
                return p;
            }
        }
    }

    pub fn main() {
        // Display the probability that a number is prime
        // if it passes all NUM_TESTS tests.
        let probability = (1.0 - (0.5_f64).powf(NUM_TESTS as f64)) * 100.0;
        println!("Probability: {}%\n", probability);

        // Generate random primes.
        loop {
            // Get the number of digits.
            let num_digits = get_u64("# Digits (max 9): ");
            if num_digits < 1 {
                break;
            }

            // Calculate minimum and maximum values.
            let min = 10u64.pow((num_digits - 1) as u32);
            let max = 10 * min;

            // Find a prime.
            println!("Prime: {}", find_prime(min, max, NUM_TESTS));
        }
    }
}
