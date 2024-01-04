mod config;
mod edit;
mod input;
mod menu;
mod queries;

// use sqlx::SqlitePool;
use std::process::ExitCode;

use config::Config;
use edit::add_movie;
use input::get_input;
use menu::print_options;

async fn entry() -> Result<(), ()> {
    let config = Config::init();
    config.verify_config().await?;

    let movie_count = 100;

    // initialize db connection
    // let db = SqlitePool::connect(&config.sql_url).await.unwrap();

    loop {
        let mut input_text = String::new();
        print_options(movie_count);
        get_input(">", &mut input_text);

        match input_text.as_str() {
            "a" => {
                add_movie(&db).await?;
            }
            "e" => {
                print!("Edit movie");
            }
            "r" => {
                print!("Recent movies");
            }
            "v" => {
                print!("View all movies");
            }
            "q" => break,
            _ => println!("Not a command"),
        }
    }

    Ok(())
}

#[async_std::main]
async fn main() -> ExitCode {
    match entry().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
