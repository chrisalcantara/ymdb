mod config;
mod edit;
mod export;
mod input;
mod menu;
mod movie;
mod queries;
mod view;

use sqlx::SqlitePool;
use std::process::ExitCode;

use config::Config;
use edit::{add_movie, edit_movie};
use input::get_input;
use menu::print_options;
use movie::Movie;
use queries::{get_all_movies, get_movie_count, get_recent_movies};

async fn entry() -> Result<(), ()> {
    let config = Config::init();
    config.verify_config().await?;

    // initialize db connection
    let db = SqlitePool::connect(&config.sql_url).await.unwrap();

    let mut movies: Vec<Movie> = [].to_vec();

    loop {
        let movie_count = get_movie_count(&db).await;
        print_options(movie_count);

        let input_text = get_input(">");

        match input_text.as_str() {
            "a" => {
                add_movie(&db).await?;
            }
            "d" => {
                edit_movie(&db, "delete").await?;
                println!("{:#?}", "DELETED");
            }
            "e" => {
                edit_movie(&db, "edit").await?;
            }
            "r" => {
                movies = get_recent_movies(&db).await;
            }
            "v" => {
                movies = get_all_movies(&db, true).await;
            }
            "x" => {
                movies = match movies.is_empty() {
                    true => get_all_movies(&db, false).await,
                    _ => movies,
                };
                export::export_data(&movies);
            }
            "q" => break,
            _ => println!("Not a command"),
        }
    }
    db.close().await;
    Ok(())
}

#[async_std::main]
async fn main() -> ExitCode {
    match entry().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
