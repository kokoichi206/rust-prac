pub mod di;
pub mod handler;
pub mod repository;
pub mod usecase;

use std::process;

const SERVER_ADR: &str = "127.0.0.1:8888";

fn main() {
    let app_module: di::AppModule;

    let am_result = di::AppModule::new();
    match am_result {
        Ok(am) => {
            println!("AppModule created");
            app_module = am;
        }
        Err(e) => {
            println!("AppModule error: {}", e);
            process::exit(1);
        }
    }

    if let Err(e) = handler::run(app_module, SERVER_ADR.to_string()) {
        println!("Server error: {}", e);
    }
}
