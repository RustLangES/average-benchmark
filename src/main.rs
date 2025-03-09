use std::hint::black_box;
use std::time::Instant;
use std::thread;
use num_cpus;

const ITERATIONS: u128 = 20_000_000_000;

fn compute_range(start: u128, end: u128) -> u128 {
    let mut sum: u128 = 0;
    for i in start..end {
        sum = sum.wrapping_add(black_box(i.wrapping_mul(i))); // black_box previene optimización agresiva
    }
    sum
}

fn benchmark_single_thread() {
    let start = Instant::now();
    let result = compute_range(0, ITERATIONS);
    let duration = start.elapsed();
    
    println!("Single-thread result: {}", result);
    println!("Single-thread time: {:.2?}", duration);
}

fn benchmark_multi_thread(num_threads: usize) {
    let chunk_size = ITERATIONS / num_threads as u128;
    let mut handles = Vec::new();
    let start = Instant::now();

    for i in 0..num_threads {
        let range_start = i as u128 * chunk_size;
        let range_end = if i == num_threads - 1 {
            ITERATIONS
        } else {
            (i as u128 + 1) * chunk_size
        };

        let handle = thread::spawn(move || compute_range(range_start, range_end));
        handles.push(handle);
    }

    let mut total_sum: u128 = 0;
    for handle in handles {
        total_sum = total_sum.wrapping_add(handle.join().unwrap());
    }
    let duration = start.elapsed();
    
    println!("Multi-thread result: {}", total_sum);
    println!("Multi-thread time: {:.2?}", duration);
}

fn main() {
    println!("CPU Benchmark in Rust");

    benchmark_single_thread();

    let num_threads = num_cpus::get();
    println!("Core numbers detected: {}", num_threads);

    benchmark_multi_thread(num_threads);
}
