use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use tera::{Context, Tera};

use crate::{
    db_connector,
    models::{Project, Technology},
};

#[get("/technologies/{tech_name}")]
async fn get_technology_page(tech_name: web::Path<String>) -> impl Responder {
    let conn = db_connector::create_connection();
    let path = tech_name.to_string();
    let (tech, projects) =
        db_connector::get_technology_page_by_url_name(&conn, &path).expect("NOT FOUND!");

    let contents = render_technology_page(tech, projects).expect("Internal Server Error");

    HttpResponse::Ok().content_type("text/html").body(contents)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"status\": \"ok\"}")
}

pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).service(get_technology_page))
        .bind((addr, port))?
        .run()
        .await
}

fn render_technology_page(tech: Technology, projects: Vec<Project>) -> Result<String, tera::Error> {
    let tmpl = Tera::new("templates/**/*").unwrap();
    let mut ctx = Context::new();

    ctx.insert("tech", &tech);
    ctx.insert("projects", &projects);

    tmpl.render("tech_page.html", &ctx)
}
