use serde_json::json;
use serde_json::Value;
use sysinfo::System;

pub fn get_system_info(score_single_thread: f64, score_multi_thread: f64) -> Value {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let system_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let system_os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let combined_system_info = format!("{} {}", system_name, system_os_version);

    json!({
        "system_info": combined_system_info,
        "system_host_name": System::host_name(),
        "number_of_cpus": sys.cpus().len(),
        "cpu_vendor_id": sys.cpus().first().map(|cpu| cpu.vendor_id()),
        "cpu_brand": sys.cpus().first().map(|cpu| cpu.brand()),
        "cpu_frequency": sys.cpus().first().map(|cpu| cpu.frequency()),
        "score_single_thread": format!("{:.3}", score_single_thread),
        "score_multi_thread": format!("{:.3}", score_multi_thread),
    })
} 