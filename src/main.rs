use std::io;
use num_cpus;
use std::hint::black_box;
use std::thread;
use std::time::Instant;
use reqwest::header::HeaderMap;
use serde_json::Value;
use serde_json::json;
use chrono::DateTime;

const ITERATIONS: u128 = 1_000_000_000;

fn compute_range(start: u128, end: u128) -> u128 {
    let mut sum: u128 = 0;
    for i in start..end {
        // Performs a * b + c operation (i * i + i) with wrapping and black_box to accurately benchmark cpu performance.
        sum = sum.wrapping_add(black_box(i.wrapping_mul(i).wrapping_add(i)));
    }
    sum
}

fn compute_score(benchmark_time: std::time::Duration) -> f64 {
    let time_in_seconds = benchmark_time.as_secs_f64();
    if time_in_seconds > 0.0 {
        (ITERATIONS as f64 / time_in_seconds) / 1_000_00.0
    } else {
        0.0
    }
}

fn benchmark_single_thread() -> f64 {
    let start = Instant::now();
    let result = compute_range(0, ITERATIONS);
    let duration = start.elapsed();
    let score: f64 = compute_score(duration);

    println!(
        "\x1B[33mSingle-thread time\x1B[0m: \x1B[36m{:.3}s\x1B[0m",
        duration.as_secs_f64()
    );
    println!("\x1B[33mSingle-thread result\x1B[0m: {:.3e}", result);
    println!("\x1B[32mSingle-thread score\x1B[0m: {}", format!("{:.3}", score));
    return score;
}

fn benchmark_multi_thread(num_threads: usize) -> f64 {
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
    println!("\x1B[32mMulti-thread score\x1B[0m: {}", format!("{:.3}", score));

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

fn ask_to_send() -> bool {
    println!("¿Desea enviar los datos de la prueba al servidor? (y/n)");

    let mut respuesta: String = String::new();

    loop {
        match io::stdin().read_line(&mut respuesta) {
            Ok(_) => {
                let input = respuesta.trim().to_lowercase();
                if input == "y" {
                    return true;
                } else if input == "n" {
                    return false;
                } else {
                    println!("Entrada no válida. Por favor, presione 'y' para sí o 'n' para no.");
                    respuesta.clear();
                }
            }
            Err(_) => {
                println!("Error al leer la entrada. Intente nuevamente.");
                respuesta.clear();
            }
        }
    }
}

async fn send_data(score_single_thread: f64, score_multi_thread: f64) -> Result<(), Box<dyn std::error::Error>> {
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let system_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let system_os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let combined_system_info = format!("{} {}", system_name, system_os_version);

    let system_info = json!({
        "system_info": combined_system_info,
        "system_host_name": System::host_name(),
        "number_of_cpus": sys.cpus().len(),
        "cpu_vendor_id": sys.cpus().first().map(|cpu| cpu.vendor_id()),
        "cpu_brand": sys.cpus().first().map(|cpu| cpu.brand()),
        "cpu_frequency": sys.cpus().first().map(|cpu| cpu.frequency()),
        "score_single_thread": format!("{:.3}", score_single_thread),
        "score_multi_thread": format!("{:.3}", score_multi_thread),
    });

    let client = reqwest::Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let request = client
    .request(reqwest::Method::POST, "http://localhost:8080/submit-tests")
    .headers(headers)
        .json(&system_info);

    let response = request.send().await?;
    let body = response.text().await?;
    let parsed: Value = serde_json::from_str(&body)?;

    let success = parsed["success"].as_bool() == Some(true);
    let message_type = if success { "Message" } else { "Error" };
    let color_code = if success { "32" } else { "31" };
    let content = if success {
        parsed["message"].as_str().unwrap_or("Success")
    } else {
        parsed["error"].as_str().unwrap_or("Unknown error")
    };

    let timestamp = parsed["timestamp"]
        .as_str()
        .and_then(|ts_str| DateTime::parse_from_rfc3339(ts_str).ok())
        .map(|ts| ts.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "N/A".to_string());

    println!("\x1B[{}m{}: {}\x1B[0m", color_code, message_type, content);
    println!("\x1B[{}mTimestamp: {}\x1B[0m", color_code, timestamp);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let baner: &str = r#"
                                                                                      #++++**
     _____ ______ _   _  ______                 _                          _        -*----#
    /  __ \| ___ \ | | | | ___ \               | |                        | |       *----#
    | /  \/| |_/ / | | | | |_/ / ___ _ __   ___| |__  _ __ ___   __ _ _ __| | __   -*---*+=== 
    | |    |  __/| | | | | ___ \/ _ \ '_ \ / __| '_ \| '_ ` _ \ / _` | '__| |/ /   *-------*=
    | \__/\| |   | |_| | | |_/ /  __/ | | | (__| | | | | | | | | (_| | |  |   <    *++++--*:
     \____/\_|    \___/  \____/ \___|_| |_|\___|_| |_|_| |_| |_|\__,_|_|  |_|\_\       *==*
                                                                                     -*++  
                                                                                     ##:
                                                                                    =#"#;
    println!("\x1B[34m{}\x1B[0m", baner);
    let score_single_thread: f64 = benchmark_single_thread();

    let num_threads: usize = num_cpus::get();
    println!(
        "\x1B[33mLogical cores numbers detected\x1B[0m: \x1B[36m{}\x1B[0m",
        num_threads
    );
    let score_multi_thread: f64 = benchmark_multi_thread(num_threads);
    if ask_to_send() {
        send_data(score_multi_thread, score_single_thread).await?;
    } else {
        println!("Datos no enviados.");
    }

    Ok(())
}
