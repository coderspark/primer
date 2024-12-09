use std::time::Instant;

/// Optimized for performance.
pub fn is_prime(n: u64) -> bool {
    // Handle edge cases.
    if n < 2 {
        return false; // 0 and 1 are not prime.
    }
    if n == 2 {
        return true; // 2 is the only even prime number.
    }

    // Test divisors from 3 to sqrt(n), skipping even numbers.
    let upper_limit = (n as f64).sqrt() as u64 + 1;
    for i in (3..upper_limit).step_by(2) {
        if n % i == 0 {
            return false; // Found a divisor, not prime.
        }
    }

    true // No divisors found, n is prime.
}

fn main() {
    let mut num = 1;
    let mut count = 0;
    let now = Instant::now();

    while now.elapsed().as_millis() >= 1000 {
        if is_prime(num) {
            count += 1;
        }
        num += 2;
    }

    println!(
        "Calculated {count} primes in {}ms",
        now.elapsed().as_millis()
    );
}
