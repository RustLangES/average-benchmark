use std::hint::black_box;
use std::thread;
use std::time::Instant;

pub const ITERATIONS: u128 = 1_000_000_000;

pub fn compute_range(start: u128, end: u128) -> u128 {
    let mut sum: u128 = 0;
    for i in start..end {
        // Performs a * b + c operation (i * i + i) with wrapping and black_box to accurately benchmark cpu performance.
        sum = sum.wrapping_add(black_box(i.wrapping_mul(i).wrapping_add(i)));
    }
    sum
}

pub fn compute_score(benchmark_time: std::time::Duration) -> f64 {
    let time_in_seconds = benchmark_time.as_secs_f64();
    if time_in_seconds > 0.0 {
        (ITERATIONS as f64 / time_in_seconds) / 1_000_00.0
    } else {
        0.0
    }
}

pub fn benchmark_single_thread() -> f64 {
    let start = Instant::now();
    let result = compute_range(0, ITERATIONS);
    let duration = start.elapsed();
    let score: f64 = compute_score(duration);

    println!(
        "\x1B[33mSingle-thread time\x1B[0m: \x1B[36m{:.3}s\x1B[0m",
        duration.as_secs_f64()
    );
    println!("\x1B[33mSingle-thread result\x1B[0m: {:.3e}", result);
    println!(
        "\x1B[32mSingle-thread score\x1B[0m: {}",
        format!("{:.3}", score)
    );
    return score;
}

pub fn benchmark_multi_thread(num_threads: usize) -> f64 {
    let chunk_size: u128 = ITERATIONS / num_threads as u128;
    let mut handles: Vec<thread::JoinHandle<(u128, std::time::Duration, usize)>> = Vec::new();
    let start: Instant = Instant::now();

    for i in 0..num_threads {
        let range_start = i as u128 * chunk_size;
        let range_end = if i == num_threads - 1 {
            ITERATIONS
        } else {
            (i as u128 + 1) * chunk_size
        };

        let handle = thread::spawn(move || {
            let thread_start = Instant::now();
            let result = compute_range(range_start, range_end);
            let duration = thread_start.elapsed();
            (result, duration, i)
        });

        handles.push(handle);
    }

    let mut total_sum: u128 = 0;
    let mut min_time = std::time::Duration::MAX;
    let mut max_time = std::time::Duration::ZERO;
    let mut min_thread = 0;
    let mut max_thread = 0;

    for handle in handles {
        let (result, duration, thread_id) = handle.join().unwrap();
        total_sum = total_sum.wrapping_add(result);

        if duration < min_time {
            min_time = duration;
            min_thread = thread_id;
        }
        if duration > max_time {
            max_time = duration;
            max_thread = thread_id;
        }
    }

    let duration = start.elapsed();
    let score: f64 = compute_score(duration);

    println!(
        "\x1B[33mMulti-thread total time\x1B[0m: \x1B[36m{:.2?}\x1B[0m",
        duration
    );
    println!("\x1B[33mMulti-thread result\x1B[0m: {:.3e}", total_sum);
    println!(
        "\x1B[32mMulti-thread score\x1B[0m: {}",
        format!("{:.3}", score)
    );

    println!(
        "\x1B[33mMin thread time\x1B[0m: {:.2?} (\x1B[32mThread {}\x1B[0m)",
        min_time, min_thread
    );
    println!(
        "\x1B[33mMax thread time\x1B[0m: {:.2?} (\x1B[32mThread {}\x1B[0m)",
        max_time, max_thread
    );
    return score;
}
