use std::hint::black_box;
use std::time::Instant;

use rayon::prelude::*;

pub const ITERATIONS: usize = 10_000_000_000;

#[inline]
/// Performs a * b + c operation (i * i + i)
/// with wrapping and black_box to accurately benchmark cpu performance.
pub fn compute(sum: &mut usize, value: usize) {
    *sum = sum.wrapping_add(black_box(value.wrapping_mul(value).wrapping_add(value)));
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

    let mut result = 0;
    for i in 0..ITERATIONS {
        compute(&mut result, i);
    }

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
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();

    let mut total_sum = 0usize;
    let mut min_time = std::time::Duration::MAX;
    let mut max_time = std::time::Duration::ZERO;
    let mut min_thread = 0;
    let mut max_thread = 0;
    let mut thread_id = 0;

    let duration = pool.install(|| {
        let (tx, rx) = std::sync::mpsc::channel();

        let start: Instant = Instant::now();

        (0..ITERATIONS)
            .into_par_iter()
            .fold(
                || (0usize, Instant::now()),
                |mut x, i| {
                    compute(&mut x.0, i);
                    x
                },
            )
            .for_each(|x| {
                let _ = tx.send(x);
            });

        let duration = start.elapsed();

        while let Ok((i, instant)) = rx.try_recv() {
            total_sum = total_sum.wrapping_add(i);

            let elapsed = instant.elapsed();

            if elapsed < min_time {
                min_time = duration;
                min_thread = thread_id;
            }

            if elapsed > max_time {
                max_time = duration;
                max_thread = thread_id;
            }

            thread_id += 1;
        }

        duration
    });

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
        "\x1B[33mAverage iterations per thread\x1B[0m: {:.2?}",
        ITERATIONS / thread_id
    );
    println!(
        "\x1B[33mMin thread time\x1B[0m: {:.2?} (\x1B[32mThread {}\x1B[0m)",
        min_time, min_thread
    );
    println!(
        "\x1B[33mMax thread time\x1B[0m: {:.2?} (\x1B[32mThread {}\x1B[0m)",
        max_time, max_thread
    );

    score
}
