use std::io;
use std::io::Write;

mod sieve;

fn gcd(mut a: u64, b: u64) -> u64 {
    a = a.rem_euclid(b);
    if a == 0 {
        return b;
    }
    gcd(b, a)
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

// Prompt the user for an u64.
fn get_u64(prompt: &str) -> u64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<u64>().expect("Error parsing integer");
}

// Perform fast exponentiation.
fn fast_exp(a: u64, b: u64) -> u64 {
    let mut r: u128 = 1;
    let a128: u128 = a.try_into().unwrap();
    let reversed_b = b.reverse_bits();
    let bits = (0..64).map(|x| reversed_b.rotate_right(x) & 0b1);
    for bit in bits {
        r *= r;
        if bit == 1 {
            r *= a128;
        }
    }
    r.try_into().unwrap()
}

// Perform fast exponentiation in a modulus.
fn fast_exp_mod(a: u64, b: u64, m: u64) -> u64 {
    let mut r: u64 = 1;
    let reversed_b = b.reverse_bits();
    let bits = (0..64).map(|x| reversed_b.rotate_right(x) & 0b1);
    for bit in bits {
        r = (r * r) % m;
        if bit == 1 {
            r = (r * a) % m;
        }
    }
    r
}

fn main() {
    sieve::sieve::main();
    loop {
        let a = get_u64("Provide the first number\n");
        let b = get_u64("Provide the second number\n");
        assert_eq!(fast_exp(a, b), a.pow(b.try_into().unwrap()));
        let m = get_u64("Provide mod\n");
        assert_eq!(fast_exp_mod(a, b, m), a.pow(b.try_into().unwrap()) % m);
    }
}
