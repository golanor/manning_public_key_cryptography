use std::io;
use std::io::Write;

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

// Prompt the user for an i64.
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

fn main() {
    loop {
        let a = get_u64("Provide the first number\n");
        let b = get_u64("Provide the second number\n");
        println!("GCD: {}", gcd(a, b));
        println!("LCM: {}", lcm(a, b));
    }
}
