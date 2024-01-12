use sqlx::{Pool, Sqlite};

use crate::{
    input::get_input,
    menu::print_results,
    movie::{FullMovie, Movie},
    queries::{ALL_MOVIES, ALL_MOVIES_WITH_ID},
    view::query_movies,
};

pub async fn add_movie(db: &Pool<Sqlite>) -> Result<(), ()> {
    let mut movie = Movie::new();
    movie.add_entry();

    let query_result =
        sqlx::query("INSERT INTO movies(title, genre, rating) VALUES ($1, $2, $3) RETURNING *")
            .bind(movie.title)
            .bind(movie.genre)
            .bind(movie.rating)
            .execute(db)
            .await;

    if query_result.is_err() {
        eprintln!(
            "ERROR: Couldn't get insert movie: {:#?}",
            query_result.unwrap()
        );
        return Err(());
    }
    Ok(())
}

pub async fn edit_movie(db: &Pool<Sqlite>, action: &str) -> Result<(), ()> {
    let title = get_input("Enter title:");

    let query = format!("{} WHERE title LIKE '%{}%'", ALL_MOVIES_WITH_ID, title);
    let r = query_movies::<FullMovie>(db, query.as_str()).await;

    // print no results found message
    print_results::<FullMovie>(&r.unwrap());

    let id = get_input("\nEnter id to edit:");

    let query_for_id = format!("{} WHERE id='{}'", ALL_MOVIES, id);
    let r_no_id = query_movies::<Movie>(db, query_for_id.as_str()).await;

    let m = &mut r_no_id.unwrap()[0];

    match action {
        "edit" => {
            m.update();
            let update_query = format!(
                "UPDATE movies SET title='{}', genre='{}', rating='{}' WHERE id='{}'",
                m.title, m.genre, m.rating, id
            );
            let _ = query_movies::<Movie>(db, update_query.as_str()).await;
            Ok(())
        }
        "delete" => {
            let delete_query = format!("DELETE FROM  movies WHERE id='{}'", id);
            let _ = query_movies::<Movie>(db, delete_query.as_str()).await;
            Ok(())
        }
        _ => Ok(()),
    }
}
