use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CpuInfo {
    pub cpu_brand: String,
    pub cpu_frequency: u32,
    pub cpu_vendor_id: String,
    pub number_of_cpus: u32,
    pub score_multi_thread: String,
    pub score_single_thread: String,
    pub system_host_name: String,
    pub system_info: String,
} 