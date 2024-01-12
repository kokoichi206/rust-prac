mod handler;

const SERVER_ADR: &str = "127.0.0.1:8888";

fn main() {
    if let Err(e) = handler::run(SERVER_ADR.to_string()) {
        println!("Server error: {}", e);
    }
}
