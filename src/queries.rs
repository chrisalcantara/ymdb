use sqlx::{Pool, Sqlite};

use crate::menu::print_results;
use crate::movie::{Movie, MovieCount};
use crate::view::query_movies;

pub const CREATE_MOVIES_TABLE: &str =
    "CREATE TABLE movies (id INTEGER PRIMARY KEY,title VARCHAR(255),genre VARCHAR(255), rating INTEGER);";

pub const TOTAL_MOVIES: &str = "SELECT COUNT(title) as count FROM movies";

pub const RECENT_MOVIES: &str =  "SELECT title,genre,rating FROM (SELECT * FROM movies ORDER BY id DESC LIMIT 5) m ORDER BY m.id ASC";

pub const ALL_MOVIES: &str = "SELECT title,genre,rating FROM movies";

pub const ALL_MOVIES_WITH_ID: &str = "SELECT id,title,genre,rating FROM movies";

pub async fn get_movie_count(db: &Pool<Sqlite>) -> i16 {
    let result = query_movies::<MovieCount>(db, TOTAL_MOVIES).await;

    result
        .expect("ERROR: Could not get count")
        .first()
        .unwrap()
        .count
}

pub async fn get_recent_movies(db: &Pool<Sqlite>) -> Vec<Movie> {
    let results = query_movies::<Movie>(db, RECENT_MOVIES).await;
    let movies = results.unwrap();
    print_results::<Movie>(&movies.clone());
    movies
}

pub async fn get_all_movies(db: &Pool<Sqlite>, show: bool) -> Vec<Movie> {
    let results = query_movies::<Movie>(db, ALL_MOVIES).await;
    let movies = results.unwrap();

    match show {
        true => {
            print_results::<Movie>(&movies.clone());
            movies
        }
        false => movies,
    }
}
