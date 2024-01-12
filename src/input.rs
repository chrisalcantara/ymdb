use std::io;
use std::io::Write;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

pub fn get_input(prompt: &str) -> String {
    let mut input_text = String::new();
    print!("{} ", prompt);
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read from stdin");
    trim_newline(&mut input_text);
    input_text
}

pub fn get_update_input(prompt: &str, movie_value: &str) -> Option<String> {
    let mut entry = String::new();
    print!("{} ({}): ", prompt, movie_value);
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read from stdin");
    trim_newline(&mut entry);

    if entry.is_empty() {
        return None;
    }
    Some(entry)
}
