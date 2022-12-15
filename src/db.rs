use crate::model::Movie;
use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;

pub struct PostgresDB {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresDB {
    pub async fn new() -> Result<PostgresDB, sqlx::Error> {
        Ok(PostgresDB {
            pool: PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://axum:axum@localhost/axum_movies")
                .await?,
        })
    }
}

#[async_trait]
pub trait DB {
    async fn query(&self, query: String) -> i64;
    async fn get_all_movies(&self) -> Vec<Movie>;
    async fn import_movies(&self, movies: Vec<Movie>);
}

#[async_trait]
impl DB for PostgresDB {
    async fn query(&self, query: String) -> i64 {
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.pool)
            .await
            .unwrap();
        row.0
    }

    async fn get_all_movies(&self) -> Vec<Movie> {
        let row = sqlx::query_as::<_, Movie>("SELECT * from movies").fetch_all(&self.pool);
        row.await.unwrap()
    }

    async fn import_movies(&self, movies: Vec<Movie>) {
        for movie in movies {
            sqlx::query(
                r#"INSERT INTO public.movies (id, title) VALUES ($1, $2);
                "#,
            )
            .bind(movie.id)
            .bind(movie.title)
            .execute(&self.pool)
            .await
            .unwrap();
        }
    }
}
