use clearscreen::clear;

pub fn print_options(count: i16) {
    println!(
        "
+--------------------------------------+
  ymdb                Total movies: {}
+--------------------------------------+
[a] Add movie      [e] Edit movie
[r] Last 5 movies  [v] View all movies
[x] Export data    [q] Quit
",
        count
    )
}

pub fn print_results<MovieType: tabled::Tabled>(results: &Vec<MovieType>) {
    clear().unwrap();
    let table = tabled::Table::new(results).to_string();
    print!("{table}");
}
