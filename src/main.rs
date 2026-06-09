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
}

fn main() {
    println!("Hello, world!");
}
