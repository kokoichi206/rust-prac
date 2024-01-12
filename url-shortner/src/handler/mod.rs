use actix_web::{web, App, HttpServer};

pub mod health;

#[actix_web::main]
pub async fn run(addr: String) -> std::io::Result<()> {
    println!("Server running at http://{}", addr);

    HttpServer::new(|| App::new().configure(routes))
        .bind(addr)?
        .run()
        .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/health").route(web::get().to(health::health)));
}
