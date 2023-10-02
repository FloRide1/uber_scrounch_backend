use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub mod command;
pub mod delivery;
pub mod location;
pub mod oauth;
pub mod product;
pub mod request;
pub mod response;
pub mod user;

#[derive(Debug)]
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
