mod config;
mod input;
mod menu;
mod queries;

use config::Config;
use input::get_input;
use menu::print_options;
use std::process::ExitCode;

async fn entry() -> Result<(), ()> {
    let config = Config::init();
    config.verify_config().await?;

    let movie_count = 100;

    loop {
        let mut input_text = String::new();
        print_options(movie_count);
        get_input(">", &mut input_text);

        match input_text.as_str() {
            "a" => {
                print!("Add movie");
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
