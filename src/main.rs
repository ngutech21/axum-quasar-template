use axum::{extract::State, response::Html, routing::get, Json, Router};
use db::PostgresDB;
use db::DB;
use model::Movie;
use std::{net::SocketAddr, sync::Arc};
use tracing::{event, Level};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod model;

type DBState = State<Arc<Box<dyn DB + Send + Sync>>>;

#[tokio::main]
async fn main() {
    let db = PostgresDB::new().await.unwrap();
    let result = db.get_all_movies().await;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let line = format!("movies {:?}", result);
    event!(Level::INFO, line);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app(Box::new(db)).into_make_service())
        .await
        .unwrap();
}

fn app(db: Box<dyn DB + Send + Sync>) -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/movies", get(get_movies))
        .route("/import_movies", get(import_movies))
        .with_state(Arc::new(db))
        .layer(TraceLayer::new_for_http())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
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
                    .uri("/movies")
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
