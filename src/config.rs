use home::home_dir;
use sqlx::SqlitePool;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;
use std::string::String;

use crate::queries::CREATE_MOVIES_TABLE;

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub db_folder: PathBuf,
    pub db_file: PathBuf,
    pub sql_url: String,
}

fn get_absolute_path(dest: &str) -> PathBuf {
    let mut path = home_dir().unwrap();
    let d = PathBuf::from(dest);
    path.push(d);
    path
}

impl Config {
    pub fn init() -> Config {
        let mut sql_url = String::from("sqlite://");
        let db_folder = get_absolute_path(".config/ymdb");
        let db_file = get_absolute_path(".config/ymdb/ymdb.db");

        let db_file_path_str = db_file.to_str().unwrap();
        sql_url.push_str(db_file_path_str);

        Config {
            db_folder,
            db_file,
            sql_url,
        }
    }

    pub async fn verify_config(&self) -> Result<(), ()> {
        self.verify_or_create_config_folder();

        let result = self.verify_or_create_database_file().await;

        match result.is_err() {
            true => Err(()),
            false => Ok(()),
        }
    }

    fn verify_or_create_config_folder(&self) {
        if !self.db_folder.exists() {
            create_dir_all(self.db_folder.as_path()).unwrap();
        }
    }

    async fn verify_or_create_database_file(&self) -> Result<(), &'static str> {
        match self.db_file.exists() {
            true => Ok(()),
            false => {
                let _ = File::create(self.db_file.as_path());
                let db = SqlitePool::connect(&self.sql_url).await.unwrap();
                let create_table_result = sqlx::query(CREATE_MOVIES_TABLE).execute(&db).await;
                if create_table_result.is_err() {
                    return Err("ERROR: Couldn't create movies table");
                }
                db.close().await;
                Ok(())
            }
        }
    }
}
