pub mod health;
pub mod url;

use crate::di;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
pub async fn run(app_module: di::AppModule, addr: String) -> std::io::Result<()> {
    println!("Server running at http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_module.clone()))
            .configure(routes)
    })
    .bind(addr)?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/health").route(web::get().to(health::health)));
    app.service(web::resource("/{shortURL}").route(web::get().to(url::get_original_url)));
}
