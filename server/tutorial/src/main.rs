#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("running server...");
    webapi_mvp::apis::create_app("0.0.0.0", 8080).await
}
