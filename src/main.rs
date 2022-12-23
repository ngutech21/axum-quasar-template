use axum::extract::Path;
use axum::{extract::State, response::Html, routing::get, Json, Router};
use axum_extra::routing::SpaRouter;
use db::PostgresDB;
use db::DB;
use hyper::Method;
use model::{AxumQuasarError, Movie};
use std::sync::Arc;
use tracing::{event, Level};

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod model;

type DBState = State<Arc<Box<dyn DB + Send + Sync>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = PostgresDB::new().await?;
    db.migrate().await;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    event!(Level::INFO, "startup");

    let addr = "[::]:8080".parse()?;
    event!(Level::INFO, "listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(
            app(Box::new(db))
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET]),
                )
                .into_make_service(),
        )
        .await?;
    Ok(())
}

fn app(db: Box<dyn DB + Send + Sync>) -> Router {
    Router::new()
        .route("/api/v1/movies", get(get_movies))
        .route("/api/v1/movie/:id", get(get_movie))
        .route("/api/v1/import_movies", get(import_movies))
        .merge(SpaRouter::new("/", "frontend/dist/spa").index_file("index.html"))
        .with_state(Arc::new(db))
        .layer(TraceLayer::new_for_http())
}

async fn get_movies(State(db): DBState) -> Result<Json<Vec<Movie>>, AxumQuasarError> {
    let result = db.get_all_movies().await?;
    Ok(Json(result))
}

async fn get_movie(
    Path(id): Path<i32>,
    State(db): DBState,
) -> Result<Json<Movie>, AxumQuasarError> {
    let result = db.get_movie(id).await?;
    match result {
        Some(movie) => Ok(Json(movie)),
        None => Err(AxumQuasarError::NotFound),
    }
}

async fn import_movies(State(db): DBState) -> Result<Html<&'static str>, AxumQuasarError> {
    let result = Movie::load_dummy_data();
    db.import_movies(result).await?;
    Ok(Html("<h1>finished importing</h1>"))
}

#[cfg(test)]
mod tests {

    use super::*;
    use async_trait::async_trait;
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    struct MockDB {}

    #[async_trait]
    impl DB for MockDB {
        async fn get_all_movies(&self) -> Result<Vec<Movie>, AxumQuasarError> {
            Ok(vec![Movie {
                id: Some(666),
                release_year: 2021,
                title: "foo".to_string(),
                genres: Some(vec!["Drama".to_string()]),
            }])
        }

        async fn get_movie(&self, _id: i32) -> Result<Option<Movie>, AxumQuasarError> {
            unimplemented!()
        }

        #[allow(dead_code)]
        async fn import_movies(&self, _movies: Vec<Movie>) -> Result<(), AxumQuasarError> {
            unimplemented!()
        }

        #[allow(dead_code)]
        async fn insert_movie(&self, _movie: Movie) -> Result<(), AxumQuasarError> {
            unimplemented!()
        }
    }

    fn create_mock_db() -> Box<dyn DB + Send + Sync> {
        let db = MockDB {};
        Box::new(db)
    }

    #[tokio::test]
    async fn test_get_movies() -> anyhow::Result<()> {
        let app = app(create_mock_db());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/movies")
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await?;
        let result_data: Vec<Movie> = serde_json::from_slice(&body)?;
        dbg!(&result_data);
        assert_eq!(result_data[0].id.unwrap(), 666);
        assert_eq!(result_data[0].title, "foo");

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_movies_from_db() -> anyhow::Result<()> {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .init();

        let app = app(Box::new(PostgresDB::new().await?));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/movies")
                    .body(Body::empty())?,
            )
            .await?;

        let status = response.status();
        let body = hyper::body::to_bytes(response.into_body()).await?;
        if status != StatusCode::OK {
            dbg!(body.clone());
        }

        assert_eq!(status, StatusCode::OK);
        let result_data: Vec<Movie> = serde_json::from_slice(&body)?;
        dbg!(result_data);
        Ok(())
    }
}
