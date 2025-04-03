use actix_web::{HttpResponse, Responder};
use log::debug;
use reqwest::Client;

pub async fn health_check() -> impl Responder {
    debug!("Health check endpoint hit");
    let client = Client::new();
    let webhook_url = "https://discord.com/api/webhooks/token";
    
    let webhook_status = match client.request(reqwest::Method::OPTIONS, webhook_url)
        .header("Content-Type", "application/json")
        .send()
        .await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        };

    if webhook_status {
        debug!("Webhook is reachable");
        HttpResponse::Ok().finish()
    } else {
        debug!("Webhook is not healthy");
        HttpResponse::GatewayTimeout().finish()
    }
}