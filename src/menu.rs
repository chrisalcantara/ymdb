pub fn print_options(count: i16) {
    println!(
        "
+--------------------------------------+
| ymdb               Total movies: {} |
+--------------------------------------+
[a] Add movie      [e] Edit movie
[r] Last 5 movies  [v] View all movies
[q] Quit
",
        count
    )
}
