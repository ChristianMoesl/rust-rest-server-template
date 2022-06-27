use crate::todo_controller;
use crate::{
    basic_auth::BasicAuthAddon,
    todo_controller::{CreateTodoDto, TodoDto},
};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing, Extension, Json, Router};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

#[derive(OpenApi)]
#[openapi(
    handlers(
        todo_controller::create_todo,
        todo_controller::get_todos
    ),
    components(TodoDto, CreateTodoDto),
    modifiers(&BasicAuthAddon),
    tags(
        (name = "todo", description = "Todo items management API")
    )
)]
struct ApiDoc;

pub fn open_api() -> Router {
    let api_doc = ApiDoc::openapi();
    let config = Arc::new(Config::from("/api-doc/openapi.json"));

    Router::new()
        .route(
            "/api-doc/openapi.json",
            routing::get({
                let doc = api_doc.clone();
                move || async { Json(doc) }
            }),
        )
        .route(
            "/swagger-ui/*tail",
            routing::get(serve_swagger_ui).layer(Extension(config)),
        )
}

async fn serve_swagger_ui(
    Path(tail): Path<String>,
    Extension(state): Extension<Arc<Config<'static>>>,
) -> impl IntoResponse {
    match utoipa_swagger_ui::serve(&tail[1..], state) {
        Ok(file) => file
            .map(|file| {
                (
                    StatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            })
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
