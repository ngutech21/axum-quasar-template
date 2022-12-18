use axum::{extract::State, response::Html, routing::get, Json, Router};
use axum_extra::routing::SpaRouter;
use db::PostgresDB;
use db::DB;
use hyper::Method;
use model::Movie;
use std::sync::Arc;
use tracing::{event, Level};

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod model;

type DBState = State<Arc<Box<dyn DB + Send + Sync>>>;

#[tokio::main]
async fn main() {
    let db = PostgresDB::new().await.unwrap();
    db.migrate().await;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    event!(Level::INFO, "startup");

    let addr = "[::]:8080".parse().unwrap();
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
        .await
        .unwrap();
}

fn app(db: Box<dyn DB + Send + Sync>) -> Router {
    Router::new()
        .route("/api/v1/movies", get(get_movies))
        .route("/api/v1/import_movies", get(import_movies))
        .merge(SpaRouter::new("/", "quasar-project/dist/spa").index_file("index.html"))
        .with_state(Arc::new(db))
        .layer(TraceLayer::new_for_http())
}

async fn get_movies(State(db): DBState) -> Json<Vec<Movie>> {
    let result = db.get_all_movies().await;
    Json(result)
}

async fn import_movies(State(db): DBState) -> Html<&'static str> {
    let result = Movie::load_dummy_data();
    db.import_movies(result).await;
    Html("<h1>finished importing</h1>")
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
        async fn get_all_movies(&self) -> Vec<Movie> {
            vec![Movie {
                id: 666,
                title: "foo".to_string(),
            }]
        }

        #[allow(dead_code)]
        async fn import_movies(&self, _movies: Vec<Movie>) {
            unimplemented!()
        }
    }

    fn create_mock_db() -> Box<dyn DB + Send + Sync> {
        let db = MockDB {};
        Box::new(db)
    }

    #[tokio::test]
    async fn test_get_movies() {
        let app = app(create_mock_db());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/movies")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let result_data: Vec<Movie> = serde_json::from_slice(&body).unwrap();
        assert_eq!(result_data[0].id, 666);
        assert_eq!(result_data[0].title, "foo");

        dbg!(result_data);
    }
}
