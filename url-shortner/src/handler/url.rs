use actix_web::{web::Data, web::Path, Error, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct HealthStatus {
    status: String,
}

pub async fn get_original_url(
    path: Path<String>,
    app_module: Data<crate::di::AppModule>,
) -> Result<HttpResponse, Error> {
    // path: https://actix.rs/docs/extractors/#path
    let short_url = path.into_inner();
    match app_module
        .static_usecase()
        .search_original_url(short_url)
        .await
    {
        Ok(url) => {
            return Ok(HttpResponse::PermanentRedirect()
                .append_header(("Location", url))
                .finish());
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
