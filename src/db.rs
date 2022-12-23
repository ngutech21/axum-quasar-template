use crate::model::{AxumQuasarError, Movie};
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
                .connect(
                    &dotenvy::var("DATABASE_URL")
                        .expect("environment variable DATABASE_URL is not set"),
                )
                .await?,
        })
    }

    pub async fn migrate(&self) {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .expect("Could not run migrations");
    }

    pub async fn start_transaction(
        &self,
    ) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, sqlx::Error> {
        self.pool.begin().await
    }

    pub async fn commit_transaction(
        &self,
        transaction: sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
        transaction.commit().await
    }
}

#[async_trait]
pub trait DB {
    async fn get_all_movies(&self) -> Result<Vec<Movie>, AxumQuasarError>;
    async fn get_movie(&self, id: i32) -> Result<Option<Movie>, AxumQuasarError>;
    async fn insert_movie(&self, movie: Movie) -> Result<(), AxumQuasarError>;
    async fn import_movies(&self, movies: Vec<Movie>) -> Result<(), AxumQuasarError>;
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
struct MovieWithGenresQuery {
    movie_id: i32,
    movie_title: String,
    genre_id: Option<i32>,
    genre_name: Option<String>,
}

#[async_trait]
impl DB for PostgresDB {
    async fn get_movie(&self, id: i32) -> Result<Option<Movie>, AxumQuasarError> {
        //let rows = sqlx::query_file_as_unchecked!(Movie, "queries/get_movie.sql", id)
        let rows = sqlx::query_as("SELECT * FROM movies where id = $1;")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(rows)
    }

    async fn get_all_movies(&self) -> Result<Vec<Movie>, AxumQuasarError> {
        let s: Vec<Movie> = sqlx::query_file_as_unchecked!(Movie, "queries/get_all_movies.sql")
            .fetch_all(&self.pool)
            .await?;
        Ok(s)
    }

    async fn insert_movie(&self, movie: Movie) -> Result<(), AxumQuasarError> {
        sqlx::query(
            r#"INSERT INTO public.movies (id, title) VALUES ($1, $2);
            "#,
        )
        .bind(movie.id)
        .bind(movie.title)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn import_movies(&self, movies: Vec<Movie>) -> Result<(), AxumQuasarError> {
        let tx = self.start_transaction().await?;
        for movie in movies {
            sqlx::query(
                r#"INSERT INTO public.movies (title, genres, release_year) VALUES ($1, $2, $3);
                "#,
            )
            .bind(movie.title)
            .bind(movie.genres)
            .bind(movie.release_year)
            .execute(&self.pool)
            .await?;
        }
        self.commit_transaction(tx).await?;
        Ok(())
    }
}
