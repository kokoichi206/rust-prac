pub mod health;
pub mod url;

use actix_web::{dev::Server, web, App, HttpServer};

use crate::di;

pub fn run(app_module: di::AppModule, addr: String) -> Result<Server, std::io::Error> {
    let s = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_module.clone()))
            .configure(routes)
    })
    .bind(addr)?
    .run();

    Ok(s)
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/health").route(web::get().to(health::health)));
    app.service(web::resource("/{shortURL}").route(web::get().to(url::get_original_url)));
}
