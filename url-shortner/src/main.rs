use std::process;

// use di/mod.rs
// use handler/mod.rs
mod di;
mod handler;
mod repository;
mod usecase;
// pub mod di;
// pub mod handler;
// pub mod repository;
// pub mod usecase;
use std::sync::Arc;

const SERVER_ADR: &str = "127.0.0.1:8888";

fn main() {
    // let app_module = match di::AppModule::new() {
    //     Ok(am) => {
    //         println!("AppModule created");
    //         am
    //     }
    //     Err(e) => {
    //         println!("AppModule error: {}", e);
    //         process::exit(1);
    //     }
    // };

    // if let Err(e) = handler::run(app_module, SERVER_ADR.to_string()) {
    //     println!("Server error: {}", e);
    // }

    let database = repository::sqlite::Database::new().unwrap();
    let ucase = usecase::main::UsecaseImpl::new(Arc::new(database));

}
