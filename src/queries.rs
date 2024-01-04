use sqlx::{sqlite::SqliteRow, FromRow, Pool, Sqlite};
use std::result::Result;

use crate::movie::MovieCount;

pub const CREATE_MOVIES_TABLE: &str =
    "CREATE TABLE movies (id INTEGER PRIMARY KEY,title VARCHAR(255),genre VARCHAR(255), rating INTEGER);";

pub const TOTAL_MOVIES: &str = "SELECT COUNT(title) as count FROM movies";

pub async fn query_movies<MovieType>(
    db: &Pool<Sqlite>,
    query: &str,
) -> Result<Vec<MovieType>, sqlx::Error>
where
    MovieType: for<'a> FromRow<'a, SqliteRow> + Send + Unpin,
{
    sqlx::query_as::<_, MovieType>(query).fetch_all(db).await
}

pub async fn get_movie_count(db: &Pool<Sqlite>) -> i16 {
    let result = query_movies::<MovieCount>(&db, TOTAL_MOVIES).await;

    result
        .expect("ERROR: Could not get count")
        .first()
        .unwrap()
        .count
}
