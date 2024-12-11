use std::time::Instant;

pub fn is_prime(n: u64) -> bool {
    // Handle small cases directly.
    match n {
        0 | 1 => return false, // 0 and 1 are not prime.
        2 | 3 => return true,  // 2 and 3 are prime.
        _ if n % 2 == 0 || n % 3 == 0 => return false, // Eliminate multiples of 2 and 3.
        _ => {}
    }

    // Check divisors of the form 6k ± 1 up to √n.
    let upper_limit = (n as f64).sqrt() as u64;
    let mut i = 5;
    while i <= upper_limit {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6; // Increment by 6 to check the next 6k ± 1 numbers.
    }

    true
}

fn main() {
    let mut num = 1;
    let mut count = 0;
    let now = Instant::now();

    while now.elapsed().as_millis() < 1000 {
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
