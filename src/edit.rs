use sqlx::{Pool, Sqlite};

use crate::movie::Movie;

pub async fn add_movie(db: &Pool<Sqlite>) -> Result<(), ()> {
    let mut movie = Movie::new();
    movie.add_entry();

    let query_result =
        sqlx::query("INSERT INTO movies(title, genre, rating) VALUES ($1, $2, $3) RETURNING *")
            .bind(movie.title)
            .bind(movie.genre)
            .bind(movie.rating)
            .execute(db)
            .await;

    if query_result.is_err() {
        eprintln!(
            "ERROR: Couldn't get insert movie: {:#?}",
            query_result.unwrap()
        );
        return Err(());
    }
    Ok(())
}
