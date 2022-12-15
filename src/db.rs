use sqlx::postgres::PgPoolOptions;

use crate::model::Movie;

pub struct DB {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DB {
    pub async fn new() -> Result<DB, sqlx::Error> {
        Ok(DB {
            pool: PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://axum:axum@localhost/axum_movies")
                .await?,
        })
    }

    pub async fn query(&self, query: String) -> i64 {
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.pool)
            .await
            .unwrap();
        row.0
    }

    pub async fn get_all_movies(&self) -> Vec<Movie> {
        let row = sqlx::query_as::<_, Movie>("SELECT * from movies").fetch_all(&self.pool);
        row.await.unwrap()
    }

    pub async fn import_movies(&self, movies: Vec<Movie>) {
        for movie in movies {
            let result = sqlx::query(
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
