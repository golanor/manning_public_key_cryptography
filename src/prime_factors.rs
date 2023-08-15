pub mod prime_factors {
    use crate::sieve::sieve;
    use std::time::Instant;

    fn find_factors(mut num: u64) -> Vec<u64> {
        let mut factors = Vec::new();
        while num % 2 == 0 {
            factors.push(2);
            num /= 2;
        }
        let mut factor = 3;
        while factor <= num {
            while num % factor == 0 {
                factors.push(factor);
                num /= factor;
            }
            factor += 2;
        }
        factors
    }

    fn multiply_vector(v: Vec<u64>) -> u64 {
        v.into_iter().fold(1, |acc, e| acc * e)
    }

    fn find_factors_sieve(mut num: u64, primes: Vec<u64>) -> Vec<u64> {
        let mut factors = Vec::new();
        for prime in primes {
            if prime > num {
                break;
            }
            while num % prime == 0 {
                factors.push(prime);
                num /= prime;
            }
        }
        if num > 1 {
            factors.push(num)
        };
        factors
    }

    pub fn main() {
        let num = crate::get_u64("Number to factor: ");
        let primes_start = Instant::now();
        let primes = sieve::sieve_to_primes(sieve::sieve_of_eratosthenes(50_000_000));
        let sieve_duration = primes_start.elapsed();
        println!("Sieve creation time: {:?}", sieve_duration);
        // Find the factors the slow way.
        let start1 = Instant::now();
        let mut factors1 = find_factors(num);
        let duration1 = start1.elapsed();
        println!("find_factors: {:?} seconds", duration1);
        sieve::print_numbers(&mut factors1);
        println!("Product: {}", multiply_vector(factors1));
        println!();

        // Use the Euler's sieve to find the factors.
        let start2 = Instant::now();
        let mut factors2 = find_factors_sieve(num, primes);
        let duration2 = start2.elapsed();
        println!("find_factors_sieve: {:?} seconds", duration2);
        sieve::print_numbers(&mut factors2);
        println!("Product: {}", multiply_vector(factors2));
        println!();
    }
}
