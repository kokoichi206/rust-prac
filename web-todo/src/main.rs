mod handlers;
mod repositories;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::repositories::{TodoRepository, TodoRepositoryForMemory};
use handlers::create_todo;

use std::env;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_a_user))
        .route("/todos", post(create_todo::<T>))
        .with_state(repository)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct CreateUser {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_a_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 12987,
        name: payload.name,
    };

    (StatusCode::CREATED, Json(user))
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::{to_bytes, Body},
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let response = create_app(repository).oneshot(req).await.unwrap();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello, World!");
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder()
            .method(Method::POST)
            .uri("/users")
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{"name":"John Doe"}"#))
            .unwrap();
        let response = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("failed to convert to User instance.");
        assert_eq!(
            user,
            User {
                id: 12987,
                name: "John Doe".to_string()
            }
        );
    }
}
