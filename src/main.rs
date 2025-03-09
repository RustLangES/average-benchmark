use std::hint::black_box;
use std::time::Instant;
use std::thread;
use num_cpus;

const ITERATIONS: u128 = 20_000_000_000;

fn compute_range(start: u128, end: u128) -> u128 {
    let mut sum: u128 = 0;
    for i in start..end {
        // Performs a * b + c operation (i * i + i) with wrapping and black_box to accurately benchmark cpu performance.
        sum = sum.wrapping_add(black_box(i.wrapping_mul(i).wrapping_add(i)));
    }
    sum
}

fn benchmark_single_thread() {
    let start: Instant = Instant::now();
    let result: u128 = compute_range(0, ITERATIONS);
    let duration: std::time::Duration = start.elapsed();
    println!("\x1B[33mSingle-thread time\x1B[0m: \x1B[36m{:.3}s\x1B[0m", duration.as_secs_f64());
    println!("\x1B[33mSingle-thread result\x1B[0m: {}", result);
}

fn benchmark_multi_thread(num_threads: usize) {
    let chunk_size: u128 = ITERATIONS / num_threads as u128;
    let mut handles: Vec<thread::JoinHandle<(u128, std::time::Duration, usize)>> = Vec::new();
    let start: Instant = Instant::now();

    for i in 0..num_threads {
        let range_start: u128 = i as u128 * chunk_size;
        let range_end: u128 = if i == num_threads - 1 {
            ITERATIONS
        } else {
            (i as u128 + 1) * chunk_size
        };

        let handle: thread::JoinHandle<(u128, std::time::Duration, usize)> = thread::spawn(move || {
            let thread_start: Instant = Instant::now();
            let result: u128 = compute_range(range_start, range_end);
            let duration: std::time::Duration = thread_start.elapsed();
            (result, duration, i) // Retorna el resultado, el tiempo y el ID del hilo
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

    let duration: std::time::Duration = start.elapsed();
    println!("\x1B[33mMulti-thread total time\x1B[0m: \x1B[36m{:.2?}\x1B[0m", duration);
    println!("\x1B[33mMulti-thread result\x1B[0m: {}", total_sum);

    println!(
        "\x1B[33mMin thread time\x1B[0m: {:.2?} (\x1B[32mThread {}\x1B[0m)",
        min_time, min_thread
    );
    println!(
        "\x1B[33mMax thread time\x1B[0m: {:.2?} (\x1B[32mThread {}\x1B[0m)",
        max_time, max_thread
    );
}


fn main() {
    let baner: &str = r#"
    _____ ______ _   _  ______                 _                          _     
    /  __ \| ___ \ | | | | ___ \               | |                        | |    
    | /  \/| |_/ / | | | | |_/ / ___ _ __   ___| |__  _ __ ___   __ _ _ __| | __ 
    | |    |  __/| | | | | ___ \/ _ \ '_ \ / __| '_ \| '_ ` _ \ / _` | '__| |/ / 
    | \__/\| |   | |_| | | |_/ /  __/ | | | (__| | | | | | | | | (_| | |  |   <  
     \____/\_|    \___/  \____/ \___|_| |_|\___|_| |_|_| |_| |_|\__,_|_|  |_|\_\ 
                                                                                                                                               
                           _        ______          _                            
                          (_)       | ___ \        | |                           
                           _ _ __   | |_/ /   _ ___| |_                          
                          | | '_ \  |    / | | / __| __|                         
                          | | | | | | |\ \ |_| \__ \ |_                          
                          |_|_| |_| \_| \_\__,_|___/\__|"#;                                                    
    println!("\x1B[34m{}\x1B[0m", baner);
    benchmark_single_thread();

    let num_threads: usize = num_cpus::get();
    println!("\x1B[33mLogical cores numbers detected\x1B[0m: \x1B[36m{}\x1B[0m", num_threads);
    benchmark_multi_thread(num_threads);
}
