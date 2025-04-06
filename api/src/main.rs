use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::http::header;
use dotenv::dotenv;

mod rate_limiter;
mod models;
mod handlers;

use rate_limiter::RateLimiterMiddleware;
use handlers::{submit_tests, health_check};
use std::env;
use log::debug;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
    let webhook_url = env::var("DISCORD_WEBHOOK_URL")
        .expect("Error: DISCORD_WEBHOOK_URL is't set in .env file");
    // Rate limiter setting: 5 requests per hour
    let rate_limiter = RateLimiterMiddleware::new(5, 3600);

    HttpServer::new(move || {
        debug!("Starting new server instance");
        App::new()
        .app_data(web::Data::new(webhook_url.clone())) // Inject webhook URL
        .wrap(
            Cors::default()
                .allowed_origin("https://average-benchmark-api.rustlang-es.org")
                .allowed_origin("http://average-benchmark-api.rustlang-es.org")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::CONTENT_TYPE])
                .max_age(3600)
        )
            // The /health route is outside the rate limiter because k8s need to use it for check status.
            .route("/health", web::get().to(health_check))
            .route("/health", web::head().to(health_check))
            // Set a ratelimit for submit-tests
            .service(
                web::scope("")
                    .wrap(rate_limiter.clone())
                    .route("/submit-tests", web::post().to(submit_tests))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
