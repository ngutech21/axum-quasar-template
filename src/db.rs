use std::collections::{HashMap, HashSet};

use crate::model::{AxumQuasarError, Genre, Movie};
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
}

#[async_trait]
pub trait DB {
    async fn get_all_movies(&self) -> Result<Vec<Movie>, AxumQuasarError>;
    async fn insert_movie(&self, movie: Movie) -> Result<(), AxumQuasarError>;
    async fn import_movies(&self, movies: Vec<Movie>) -> Result<(), AxumQuasarError>;
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
struct MovieWithGenresQuery {
    movie_id: i32,
    movie_title: String,
    genre_id: i32,
    genre_name: String,
}

#[async_trait]
impl DB for PostgresDB {
    async fn get_all_movies(&self) -> Result<Vec<Movie>, AxumQuasarError> {
        Ok(
            sqlx::query_file_as!(MovieWithGenresQuery, "queries/get_all_movies.sql")
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .fold(HashMap::new(), |mut movies, row| {
                    let movie = movies.entry(row.movie_id).or_insert(Movie {
                        id: row.movie_id,
                        title: row.movie_title,
                        genres: HashSet::new(),
                    });
                    movie.genres.insert(Genre {
                        id: row.genre_id,
                        name: row.genre_name,
                    });
                    movies
                })
                .values()
                .cloned()
                .collect(),
        )
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
        for movie in movies {
            sqlx::query(
                r#"INSERT INTO public.movies (id, title) VALUES ($1, $2);
                "#,
            )
            .bind(movie.id)
            .bind(movie.title)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }
}
