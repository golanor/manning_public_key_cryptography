pub mod sieve {
    // Sieve of Eratosthenes
    pub fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
        let mut sieve = vec![true; limit + 1];
        sieve[0] = false;
        sieve[1] = false;
        for i in (4..=limit).step_by(2) {
            sieve[i] = false;
        }

        for i in (3..=limit).step_by(2) {
            if sieve[i] {
                for j in (i * i..=limit).step_by(i) {
                    sieve[j] = false;
                }
            }
        }
        sieve
    }

    fn print_sieve(sieve: &Vec<bool>) {
        let size = sieve.len();
        print!("2 ");
        for i in (3..size).step_by(2) {
            if sieve[i] {
                print!("{} ", i)
            }
        }
        println!();
    }

    pub fn sieve_to_primes(sieve: Vec<bool>) -> Vec<u64> {
        sieve
            .into_iter()
            .enumerate()
            .filter(|(_, b)| *b)
            .map(|(i, _)| i as u64)
            .collect::<Vec<u64>>()
    }

    // Print the vector of numbers.
    pub fn print_numbers(primes: &mut Vec<u64>) {
        for prime in primes {
            print!("{prime} ");
        }
        println!();
    }

    pub fn main() {
        let max = crate::get_u64("Max: ");
        let mut sieve = sieve_of_eratosthenes(max as usize);
        if max < 1000 {
            print_sieve(&mut sieve);
        }

        let mut primes = sieve_to_primes(sieve);
        if max < 1000 {
            print_numbers(&mut primes);
        }
    }
}
