use crate::{input::get_input, menu::print_export_options, movie::Movie};
use clearscreen::clear;
use csv::Writer;
use std::fs::write;

fn get_file_name(ext: &str) -> String {
    let mut file_name_input = String::new();
    let f = format!("Enter file name (default: data.{})", ext);
    get_input(&f, &mut file_name_input);
    match file_name_input.as_str() {
        "" => {
            format!("./data.{}", ext)
        }
        _ => file_name_input,
    }
}

fn export_csv(movies: &Vec<Movie>) {
    let file_name = get_file_name(".csv");
    let mut writer = Writer::from_writer(vec![]);

    for movie in movies {
        writer
            .serialize(movie)
            .expect("ERROR: Couldn't serialize Movie entry.")
    }

    let data = std::string::String::from_utf8(writer.into_inner().unwrap()).unwrap();
    write(&file_name, data).expect("ERROR: Couldn't write file.");
    println!("File saved as: {}", file_name);
}

pub fn export_data(movies: &Vec<Movie>) {
    clear().expect("ERROR: Couldn't clear screen.");
    print_export_options();

    loop {
        let mut input_text = String::new();
        get_input(">", &mut input_text);
        match input_text.as_str() {
            "c" => {
                println!("export csv");
                export_csv(movies);
                break;
            }
            "j" => {
                println!("export json");
                break;
            }
            "t" => {
                println!("export tsv");
                break;
            }

            "x" => {
                println!("export txt");
                break;
            }
            "q" => break,
            _ => {
                println!("Not a command");
            }
        }
    }
}
