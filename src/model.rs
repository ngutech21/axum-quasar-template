use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Movie {
    #[serde(skip)]
    pub id: i32,
    #[serde(alias = "title")]
    pub title: String,
    #[serde(alias = "year")]
    pub release_year: i16,
    pub genres: Option<Vec<String>>,
}

impl Movie {
    pub fn load_dummy_data() -> Vec<Movie> {
        serde_json::from_str::<Vec<Movie>>(include_str!("../tmp/wikipedia-movies.json"))
            .expect("Could not parse dummy data")
    }
}

#[derive(Error, Debug)]
pub enum AxumQuasarError {
    #[error(transparent)]
    DBError(#[from] sqlx::Error),
}

impl IntoResponse for AxumQuasarError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AxumQuasarError::DBError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
