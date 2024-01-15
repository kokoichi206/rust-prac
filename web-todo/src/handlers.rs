use axum::{
    extract::{rejection::FormRejection, Form, FromRequest, Path, Request, State},
    http::StatusCode,
    response::IntoResponse,
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use validator::Validate;

use async_trait::async_trait;

use crate::repositories::{CreateTodo, TodoRepository, UpdateTodo};

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

// trait 内での async を可能にするためのマクロ。
#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = (StatusCode, String);

    // see: https://github.com/tokio-rs/axum/blob/axum-v0.7.4/examples/validator/src/main.rs
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                let message = format!("Json parse error: {}", rejection);
                (StatusCode::BAD_REQUEST, message)
            })?;

        value.validate().map_err(|rejection| {
            let message = format!("Validation error: {}", rejection);
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

pub async fn create_todo<T: TodoRepository + 'static>(
    State(repository): State<T>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todo<T: TodoRepository>(
    State(repository): State<T>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository>(State(repository): State<T>) -> impl IntoResponse {
    let todos = repository.all();
    (StatusCode::OK, Json(todos))
}

pub async fn update_todo<T: TodoRepository>(
    State(repository): State<T>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T: TodoRepository>(
    State(repository): State<T>,
    Path(id): Path<i32>,
) -> StatusCode {
    repository
        .delete(id)
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}
