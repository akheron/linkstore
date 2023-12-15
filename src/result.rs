use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = std::result::Result<T, AppError>;

pub struct AppError(eyre::Report);

impl From<eyre::Report> for AppError {
    fn from(report: eyre::Report) -> Self {
        Self(report)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
