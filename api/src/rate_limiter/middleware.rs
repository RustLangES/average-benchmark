use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, ResponseError,
};
use chrono::Utc;
use serde_json::json;
use std::fmt;
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use super::RateLimiter;

#[derive(Debug)]
struct RateLimitError;

impl fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rate limit excedido")
    }
}

impl ResponseError for RateLimitError {
    fn error_response(&self) -> HttpResponse {
        let json_error = json!({
            "success": false,
            "error": "Rate limit excedido. Por favor, inténtalo más tarde.",
            "timestamp": Utc::now().to_rfc3339()
        });

        HttpResponse::TooManyRequests()
            .content_type("application/json")
            .json(json_error)
    }
}

// Factory para el middleware
#[derive(Clone)]
pub struct RateLimiterMiddleware {
    limiter: Arc<RateLimiter>,
}

impl RateLimiterMiddleware {
    pub fn new(max_requests: usize, window_size: u64) -> Self {
        RateLimiterMiddleware {
            limiter: Arc::new(RateLimiter::new(max_requests, window_size)),
        }
    }
}

// Implementing Transform for RateLimiterMiddleware
impl<S, B> Transform<S, ServiceRequest> for RateLimiterMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimiterMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddlewareService {
            service,
            limiter: self.limiter.clone(),
        }))
    }
}

// Middleware service
pub struct RateLimiterMiddlewareService<S> {
    service: S,
    limiter: Arc<RateLimiter>,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ip = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_string();

        if self.limiter.is_rate_limited(&ip) {
            return Box::pin(async move { Err(RateLimitError.into()) });
        }

        let fut = self.service.call(req);
        Box::pin(async move { fut.await })
    }
}
