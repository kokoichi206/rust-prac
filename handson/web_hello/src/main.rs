use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

const SERVER_ADR: &str = "127.0.0.1:8888";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://{}", SERVER_ADR);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/hello", web::get().to(hello))
    })
    .bind(SERVER_ADR)?
    .run()
    .await
}

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "index page!"
}

#[derive(Serialize, Deserialize, Debug)]
struct Me {
    name: String,
    age: u8,
}

async fn hello(q: web::Query<Me>) -> Result<HttpResponse, Error> {
    println!("Query: {:?}", q);

    let me = q.0;

    // response with json.
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .json(me))

    // Ok(HttpResponse::Ok()
    //     .content_type("text/html; charset=utf-8")
    //     .body(format!("Hello {}! I'm {} years old.", q.name, q.age)))
}
