use sqlx::{sqlite::SqliteRow, FromRow, Pool, Sqlite};
use std::result::Result;

pub async fn query_movies<MovieType>(
    db: &Pool<Sqlite>,
    query: &str,
) -> Result<Vec<MovieType>, sqlx::Error>
where
    MovieType: for<'a> FromRow<'a, SqliteRow> + Send + Unpin,
{
    sqlx::query_as::<_, MovieType>(query).fetch_all(db).await
}
