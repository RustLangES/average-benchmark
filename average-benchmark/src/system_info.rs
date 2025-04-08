use serde_json::json;
use serde_json::Value;
use std::fs;
use std::io::{self, BufRead};
use std::process::Command;
use sysinfo::System;

fn run_command_output(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|output| {
            let out = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if out.is_empty() {
                None
            } else {
                Some(out)
            }
        })
}

fn get_prop(prop: &str) -> Option<String> {
    run_command_output("getprop", &[prop])
}

#[derive(Debug)]
struct CpuInfo {
    cpu_brand: Option<String>,
    cpu_frequency: Option<u64>,
    cpu_vendor_id: Option<String>,
    number_of_cpus: usize,
}

fn get_max_cpu_frequency() -> Option<u64> {
    let mut max_freq = 0;
    for i in 0..64 {
        let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_max_freq", i);
        if let Ok(freq_str) = fs::read_to_string(&path) {
            if let Ok(freq_khz) = freq_str.trim().parse::<u64>() {
                max_freq = max_freq.max(freq_khz);
            }
        }
    }
    if max_freq > 0 {
        Some(max_freq / 1000)
    } else {
        None
    }
}

fn get_cpu_info_fallback() -> CpuInfo {
    let cpuinfo_path = "/proc/cpuinfo";
    let mut cpu_brand: Option<String> = None;
    let mut cpu_vendor_id: Option<String> = None;
    let mut number_of_cpus = 0;

    if let Ok(file) = fs::File::open(cpuinfo_path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.starts_with("processor") {
                number_of_cpus += 1;
            }
            if line.starts_with("Hardware") && cpu_brand.is_none() {
                if let Some(index) = line.find(':') {
                    let value = line[index + 1..].trim();
                    if !value.is_empty() {
                        cpu_brand = Some(value.to_string());
                    }
                }
            }
        }
    }

    if number_of_cpus == 0 {
        number_of_cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
    }

    let cpu_frequency = get_max_cpu_frequency();

    if cpu_brand.is_none() {
        cpu_brand = get_prop("ro.soc.model");
    }
    cpu_vendor_id = get_prop("ro.soc.manufacturer");

    CpuInfo {
        cpu_brand,
        cpu_frequency,
        cpu_vendor_id,
        number_of_cpus,
    }
}

pub fn get_system_info(score_single_thread: f64, score_multi_thread: f64) -> Value {
    let mut sys = System::new_all();
    sys.refresh_all();

    let system_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let system_os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let combined_system_info = format!("{} {}", system_name, system_os_version);

    let sysinfo_cpus = sys.cpus();
    let num_cpus = sysinfo_cpus.len();
    let sys_cpu_brand = sysinfo_cpus.first().map(|cpu| cpu.brand().to_string());

    let fallback = get_cpu_info_fallback();

    let final_cpu_brand = sys_cpu_brand
        .filter(|s| !s.is_empty())
        .or(fallback.cpu_brand);

    let final_cpu_vendor = fallback.cpu_vendor_id;
    let final_cpu_frequency = fallback.cpu_frequency;
    let final_number_of_cpus = if num_cpus == 0 {
        fallback.number_of_cpus
    } else {
        num_cpus
    };

    let host_name = run_command_output("whoami", &[]).unwrap_or_else(|| "unknown".to_string());

    json!({
        "system_info": combined_system_info,
        "system_host_name": host_name,
        "number_of_cpus": final_number_of_cpus,
        "cpu_vendor_id": final_cpu_vendor,
        "cpu_brand": final_cpu_brand,
        "cpu_frequency": final_cpu_frequency,
        "score_single_thread": format!("{:.3}", score_single_thread),
        "score_multi_thread": format!("{:.3}", score_multi_thread),
    })
}
