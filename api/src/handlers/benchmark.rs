use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use reqwest;
use serde_json::json;
use std::env;

use crate::models::CpuInfo;

pub async fn submit_tests(info: web::Json<CpuInfo>) -> impl Responder {
    let webhook_url = env::var("DISCORD_WEBHOOK_URL").unwrap_or_else(|_| "".to_string());
    if webhook_url.is_empty() {
        return HttpResponse::InternalServerError().body("Error: DISCORD_WEBHOOK_URL no está configurada");
    }
    let timestamp = Utc::now().to_rfc3339();

    let payload = json!({
        "embeds": [{
            "title": "📢 ¡Nuevo Benchmark! 🔥",
            "color": 1127128,
            "fields": [
                {"name": "🔹 Procesador", "value": info.cpu_brand, "inline": false},
                {"name": "🔹 Núcleos lógicos", "value": info.number_of_cpus, "inline": true},
                {"name": "🔹 Frecuencia", "value": format!("{} MHz", info.cpu_frequency), "inline": true},
                {"name": "🔹 Proveedor", "value": info.cpu_vendor_id, "inline": true},
                {"name": "💻 Sistema Operativo", "value": info.system_info, "inline": false},
                {"name": "👩‍💻👨‍💻 Nombre de host", "value": info.system_host_name, "inline": true},
                {"name": "🔸 Single-thread Score", "value": info.score_single_thread.to_string(), "inline": true},
                {"name": "🔸 Multi-thread Score", "value": info.score_multi_thread.to_string(), "inline": true},
            ],
            "footer": {
                "text": "Benchmark realizado con cariño",
                "icon_url": "https://avatars.githubusercontent.com/u/74681819?s=280&v=4",
            },
            "timestamp": timestamp,
        }]
    });

    let client = reqwest::Client::new();
    let res = client.post(webhook_url).json(&payload).send().await;

    match res {
        Ok(response) if response.status().is_success() => {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                    "success": true,
                    "message": "Webhook enviado correctamente",
                    "timestamp": Utc::now().to_rfc3339()
                }))
        }
        Ok(response) => HttpResponse::InternalServerError()
            .content_type("application/json")
            .json(json!({
                "success": false,
                "error": format!("Error al enviar el webhook: {}", response.status()),
                "timestamp": Utc::now().to_rfc3339()
            })),
        Err(e) => {
            eprintln!("Error enviando el webhook: {:?}", e);
            HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(json!({
                    "success": false,
                    "error": "Error al enviar el webhook",
                    "timestamp": Utc::now().to_rfc3339()
                }))
        }
    }
} 