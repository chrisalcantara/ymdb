use serde::Serialize;
use sqlx::FromRow;
use tabled::Tabled;

use crate::input::{get_input, get_update_input};

#[derive(Clone, Default, Debug, FromRow, Tabled, Serialize)]
pub struct Movie {
    pub title: String,
    pub genre: String,
    pub rating: i8,
}

#[derive(Clone, FromRow, Debug, Tabled)]
pub struct FullMovie {
    pub id: i16,
    pub title: String,
    pub genre: String,
    pub rating: i8,
}

#[derive(Clone, FromRow, Debug, Tabled)]
pub struct MovieCount {
    pub count: i16,
}

impl Movie {
    pub fn new() -> Movie {
        Movie::default()
    }

    pub fn view(&self) {
        clearscreen::clear().unwrap();
        println!("Title: {}", self.title);
        println!("Genre: {}", self.genre);
        println!("Rating: {}", self.rating);
    }

    pub fn add_entry(&mut self) {
        loop {
            self.title = get_input("Enter title:");
            self.genre = get_input("Enter genre:");

            // ensure rating is a number
            loop {
                let rating = get_input("Enter rating:");
                let parsed_rating = rating.parse::<i8>();

                if parsed_rating.is_ok() {
                    let r = parsed_rating.unwrap();
                    self.rating = r;
                    break;
                }
                println!("Rating must be number");
            }

            self.view();

            let ans = get_input("Look good? (y/n):");
            if ans == "y" {
                break;
            }
        }
    }

    pub fn update(&mut self) {
        loop {
            let title = get_update_input("Enter title", &self.title);
            if let Some(title) = title {
                self.title = title;
            }
            let your_rating = get_update_input("Enter your rating", &self.rating.to_string());
            if let Some(your_rating) = your_rating {
                self.rating = your_rating.parse().unwrap();
            }
            let genre = get_update_input("Enter Genre", &self.genre);
            if let Some(genre) = genre {
                self.genre = genre;
            }

            self.view();
            let ans = get_input("Look good? (y/n):");

            if ans == "y" {
                break;
            }
        }
    }
}
