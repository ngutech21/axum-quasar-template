use std::collections::HashSet;

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
    pub id: i32,
    pub title: String,
    //pub year: i32,
    pub genres: HashSet<Genre>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow, Hash)]
pub struct Genre {
    pub id: i32,
    pub name: String,
}

impl Movie {
    pub fn load_dummy_data() -> Vec<Movie> {
        serde_json::from_str::<Vec<Movie>>(include_str!("../dummy_data.json"))
            .expect("Could not parse dummy data")
    }
}

#[derive(Error, Debug)]
pub enum AxumQuasarError {
    #[error("DBError: {0}")]
    DBError(#[from] sqlx::Error),
}

impl IntoResponse for AxumQuasarError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AxumQuasarError::DBError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DBError"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
