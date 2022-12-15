use axum::{extract::State, response::Html, routing::get, Json, Router};
use axum_extra::response::ErasedJson;
use model::Movie;
use std::{net::SocketAddr, sync::Arc};
use tracing::{event, span, Level};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod model;

#[tokio::main]
async fn main() {
    let db = db::DB::new().await.unwrap();
    let result = db.get_all_movies().await;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let line = format!("movies {:?}", result);
    event!(Level::INFO, line);

    let app = Router::new()
        .route("/", get(handler))
        .route("/movies", get(get_movies))
        .route("/import_movies", get(import_movies))
        .with_state(Arc::new(db))
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn get_movies(State(db): State<Arc<db::DB>>) -> Json<Vec<Movie>> {
    let result = db.get_all_movies().await;
    Json(result)
}

async fn import_movies(State(db): State<Arc<db::DB>>) -> Html<&'static str> {
    let result = Movie::load_dummy_data();
    db.import_movies(result).await;
    Html("<h1>finished importing</h1>")
}
