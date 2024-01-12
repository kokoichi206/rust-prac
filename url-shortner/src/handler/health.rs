use actix_web::{Error, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct HealthStatus {
    status: String,
}

pub async fn health() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(HealthStatus {
            status: "ok".to_string(),
        }))
}
