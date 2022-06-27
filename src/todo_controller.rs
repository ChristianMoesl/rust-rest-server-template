use crate::rest_error::RestError;
use crate::todo_service::TodoService;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::Component;
use validator::Validate;

// the input to our `create_user` handler
#[derive(Serialize, Deserialize, Component, Validate, Clone, Debug, PartialEq, Eq)]
pub struct CreateTodoDto {
    #[component(example = "Buy groceries")]
    #[validate(length(max = 100))]
    pub text: String,
}

// the input to our `create_user` handler
#[derive(Serialize, Deserialize, Component, Clone, Debug, PartialEq, Eq)]
pub struct TodoDto {
    #[component(example = "1")]
    pub id: String,
    #[component(example = "Buy groceries")]
    pub text: String,
    #[component(example = "true")]
    pub completed: bool,
}

/// Create new Todo
///
/// Tries to create a new Todo item to in-memory storage or fails with 409 conflict if already exists.
// (status = 409, description = "Todo already exists", body = TodoError)
#[utoipa::path(
    post,
    path = "/todos",
    request_body = CreateTodo,
    responses(
        (status = 201, description = "Todo item created successfully", body = Todo),
    )
)]
async fn create_todo(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateTodoDto>,
    Extension(todo_service): Extension<Arc<dyn TodoService>>,
) -> Result<(StatusCode, Json<TodoDto>), RestError> {
    payload.validate().map_err(RestError::ValidationFailed)?;

    todo_service
        .add_todo(&payload)
        .await
        .map(|todo| (StatusCode::CREATED, Json(todo)))
        .map_err(RestError::Unknown)
}

/// Get all Todo
///
/// Tries to list all todo items.
#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "List of todo items successfully", body = [Todo]),
    )
)]
async fn get_todos(
    Extension(todo_service): Extension<Arc<dyn TodoService>>,
) -> Result<(StatusCode, Json<Vec<TodoDto>>), RestError> {
    todo_service
        .list_todos()
        .await
        .map(|todos| (StatusCode::OK, Json(todos)))
        .map_err(RestError::Unknown)
}

pub fn todo() -> Router {
    Router::new().route("/todos", get(get_todos).post(create_todo))
}
