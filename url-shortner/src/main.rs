use std::process;

mod di;
mod domain;
mod handler;
mod repository;
mod usecase;

const SERVER_ADR: &str = "127.0.0.1:8888";

#[actix_web::main]
async fn main() {
    let app_module = match di::AppModule::new().await {
        Ok(am) => {
            println!("AppModule created");
            am
        }
        Err(e) => {
            println!("AppModule error: {}", e);
            process::exit(1);
        }
    };

    match handler::run(app_module, SERVER_ADR.to_string()) {
        Ok(s) => {
            println!("Server running at http://{}", SERVER_ADR);
            if let Err(e) = s.await {
                println!("Server error: {}", e);
            }
        }
        Err(e) => {
            println!("Server error: {}", e);
            process::exit(1);
        }
    }
}
