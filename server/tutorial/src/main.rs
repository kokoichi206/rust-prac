#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let port = 9911;
    println!("running server on port {port}...");
    webapi_mvp::apis::create_app("0.0.0.0", port).await
}
