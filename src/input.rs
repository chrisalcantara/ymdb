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

pub fn get_input(prompt: &str, input_text: &mut String) {
    print!("{} ", prompt);
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(input_text)
        .expect("Failed to read from stdin");
    trim_newline(input_text);
}
