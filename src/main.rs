mod benchmark;
mod system_info;
mod network;
mod utils;

use num_cpus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::display_banner();
    
    let score_single_thread: f64 = benchmark::benchmark_single_thread();

    let num_threads: usize = num_cpus::get();
    println!(
        "\x1B[33mLogical cores numbers detected\x1B[0m: \x1B[36m{}\x1B[0m",
        num_threads
    );
    
    let score_multi_thread: f64 = benchmark::benchmark_multi_thread(num_threads);
    
    if utils::ask_to_send() {
        let system_info = system_info::get_system_info(score_single_thread, score_multi_thread);
        network::send_data(system_info).await?;
    } else {
        println!("Datos no enviados.");
    }

    Ok(())
}
