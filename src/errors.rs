use axum::response::IntoResponse;
use reqwest::StatusCode;

pub enum Error {
    InternalServerError,
    TraewellingConnectionError
}

impl From<r2d2::Error> for Error {
    fn from(value: r2d2::Error) -> Self {
        println!("Database error: {:?}", value);
        Self::InternalServerError
    }
}

impl From<r2d2_sqlite::rusqlite::Error> for Error {
    fn from(value: r2d2_sqlite::rusqlite::Error) -> Self {
        println!("Database error: {:?}", value);
        Self::InternalServerError
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        println!("Connection error: {:?}", value);
        Self::TraewellingConnectionError
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, match self {
            Self::InternalServerError => "Internal server error",
            Self::TraewellingConnectionError => "Traewelling connection error"
        }).into_response()
    }
}