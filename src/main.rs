use std::f64::consts::SQRT_2;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u64;

    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("Hello, world!");
}
