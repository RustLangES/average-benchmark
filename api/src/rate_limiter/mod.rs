mod middleware;

pub use middleware::RateLimiterMiddleware;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct RateLimiter {
    // HashMap that stores the IP and the last access
    clients: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window_size: u64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_size: u64) -> Self {
        RateLimiter {
            clients: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_size,
        }
    }

    pub fn is_rate_limited(&self, ip: &str) -> bool {
        let now = Instant::now();
        let window = Duration::from_secs(self.window_size);

        let mut clients = match self.clients.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("Error al obtener el lock del Mutex: {:?}", poisoned);
                return true; // no confiamos porque ha sido poisoned, así que lo limitamos
            }
        };

        let timestamps = clients.entry(ip.to_string()).or_insert_with(Vec::new);

        // Remove timestamps that are outside the time window
        timestamps.retain(|&timestamp| now.duration_since(timestamp) < window);

        if timestamps.len() >= self.max_requests {
            return true;
        }

        timestamps.push(now);
        false
    }
}
