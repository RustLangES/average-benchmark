use actix_web::{HttpResponse, Responder};
use reqwest::Client;

pub async fn health_check() -> impl Responder {
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
        HttpResponse::Ok().json(serde_json::json!({
            "status": "ok",
            "message": "Service and webhook are healthy"
        }))
    } else {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Service is healthy but webhook is not reachable"
        }))
    }
}