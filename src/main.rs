mod config;
mod input;

use config::Config;
use input::get_input;

fn main() {
    let config = Config::init();
    config.verify_config();
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
}
