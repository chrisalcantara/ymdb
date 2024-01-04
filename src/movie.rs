use sqlx::FromRow;
use tabled::Tabled;

use crate::input::get_input;

#[derive(Clone, Default, Debug, FromRow, Tabled)]
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
        println!("Your rating: {}", self.rating);
        println!("Genre: {}", self.genre);
    }

    pub fn add_entry(&mut self) {
        loop {
            let mut title = String::new();
            let mut genre = String::new();
            let mut rating = String::new();

            let mut ans = String::new();

            get_input("Enter title:", &mut title);
            get_input("Enter your rating:", &mut rating);
            get_input("Enter genre:", &mut genre);

            self.title = title;
            self.rating = rating.parse::<i8>().unwrap();
            self.genre = genre;

            self.view();
            get_input("Look good? (y/n): ", &mut ans);

            if ans == "y" {
                break;
            }
        }
    }

    // pub fn update(&mut self) {
    //     loop {
    //         let mut ans = String::new();
    //         let title = get_update_input("Enter title", &self.title);
    //         if let Some(title) = title {
    //             self.title = title;
    //         }
    //         let your_rating = get_update_input("Enter your rating", &self.rating.to_string());
    //         if let Some(your_rating) = your_rating {
    //             self.rating = your_rating.parse().unwrap();
    //         }
    //         let genre = get_update_input("Enter Genre", &self.genre);
    //         if let Some(genre) = genre {
    //             self.genre = genre;
    //         }

    //         self.view();
    //         get_input("Look good? (y/n):", &mut ans);

    //         if ans == "y" {
    //             break;
    //         }
    //     }
    // }
}
