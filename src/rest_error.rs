use axum::body::boxed;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use validator::ValidationErrors;

#[derive(Debug)]
pub enum RestError {
    ValidationFailed(ValidationErrors),
    Unknown(anyhow::Error),
}

impl IntoResponse for RestError {
    fn into_response(self) -> Response {
        match self {
            RestError::ValidationFailed(errors) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(boxed(errors.to_string())),
            RestError::Unknown(error) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(boxed(error.to_string())),
        }
        .unwrap_or(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(boxed("could not build error response".to_string()))
                .expect("should be a valid request"),
        )
    }
}
