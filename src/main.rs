use std::thread; // ← bu ham yetishmayotgan edi

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

fn count_primes_in_range(start: u64, end: u64) -> u64 {
    (start..end).filter(|&n| is_prime(n)).count() as u64
}

fn count_primes_multi_threaded(start: u64, end: u64, num_threads: usize) -> u64 {
    let range_size = end - start;
    let chunk_size = range_size / num_threads as u64; // typo tuzatildi
    let mut handles = vec![]; // ← thread handle'larni saqlash uchun

    for i in 0..num_threads {
        let thread_start = start + (i as u64 * chunk_size);
        let thread_end = if i == num_threads - 1 {
            end
        } else {
            thread_start + chunk_size
        };

        // ← thread spawn qilamiz
        let handle = thread::spawn(move || count_primes_in_range(thread_start, thread_end));
        handles.push(handle);
    }

    // ← barcha thread natijalarini yig'amiz
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

fn main() {
    let count = count_primes_multi_threaded(0, 1_000_000, 4);
    println!("Sonlar soni: {}", count); // → 78498
}
