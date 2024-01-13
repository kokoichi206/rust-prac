use actix_web::{web::Data, Error, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct HealthStatus {
    status: String,
}

// pub async fn health(app_module: Data<crate::di::AppModule>) -> Result<HttpResponse, Error> {
pub async fn health(ucase: Data<Box<impl crate::usecase::Usecase>>) -> Result<HttpResponse, Error> {
    // match app_module.static_usecase().health() {
    match ucase.health().await {
        Ok(_) => {
            return Ok(HttpResponse::Ok()
                .content_type("application/json; charset=utf-8")
                .json(HealthStatus {
                    status: "ok".to_string(),
                }));
        }
        Err(e) => {
            println!("health error: {}", e);

            return Ok(HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .json(HealthStatus {
                    status: e.to_string(),
                }));
        }
    }
}
