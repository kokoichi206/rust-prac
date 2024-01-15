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

use crate::repositories::{TodoRepository, TodoRepositoryForDb};
use handlers::{all_todo, create_todo, delete_todo, find_todo, update_todo};

use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::debug!("Connecting to {}", db_url);
    let pool = PgPool::connect(db_url)
        .await
        .expect("Failed to connect to Postgres.");

    let repository = TodoRepositoryForDb::new(pool.clone());
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
        .route("/todos", post(create_todo::<T>).get(all_todo::<T>))
        .route(
            "/todos/:id",
            get(find_todo::<T>)
                .delete(delete_todo::<T>)
                .patch(update_todo::<T>),
        )
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
    use crate::repositories::{test_utils::TodoRepositoryForMemory, CreateTodo, Todo};
    use axum::{
        body::{to_bytes, Body},
        http::{header, Method, Request},
        response::Response,
    };
    use tower::ServiceExt;

    fn build_todo_req_with_json(path: &str, method: Method, json_body: String) -> Request<Body> {
        Request::builder()
            .method(method)
            .uri(path)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json_body))
            .unwrap()
    }

    fn build_todo_req_with_empty(path: &str, method: Method) -> Request<Body> {
        Request::builder()
            .method(method)
            .uri(path)
            .body(Body::empty())
            .unwrap()
    }

    async fn res_to_todo(res: Response) -> Todo {
        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Todo = serde_json::from_str(&body).expect("failed to convert to Todo instance.");
        todo
    }

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

    #[tokio::test]
    async fn should_created_todo() {
        // Arrange
        let expected = Todo::new(1, "should_return_created_todo".to_string());

        let repo = TodoRepositoryForMemory::new();
        let req = build_todo_req_with_json(
            "/todos",
            Method::POST,
            r#"{"text":"should_return_created_todo"}"#.to_string(),
        );

        // Act
        let res = create_app(repo).oneshot(req).await.unwrap();

        // Assert
        assert_eq!(res.status(), StatusCode::CREATED);
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_find_todo() {
        // Arrange
        let expected = Todo::new(1, "should_find_todo".to_string());

        let repo = TodoRepositoryForMemory::new();
        repo.create(CreateTodo::new("should_find_todo".to_string()))
            .await
            .unwrap();
        let req = build_todo_req_with_empty("/todos/1", Method::GET);

        // Act
        let res = create_app(repo).oneshot(req).await.unwrap();

        // Assert
        assert_eq!(res.status(), StatusCode::OK);
        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Todo = serde_json::from_str(&body)
            .expect(&format!("cannot convert to Todo instance. body: {}", body));
        assert_eq!(expected, todo);
    }
}
