mod handler;
mod di;
mod repository;
mod usecase;

const SERVER_ADR: &str = "127.0.0.1:8888";

fn main() {
    let app_module = di::AppModule::new();

    if let Err(e) = handler::run(app_module, SERVER_ADR.to_string()) {
        println!("Server error: {}", e);
    }
}
