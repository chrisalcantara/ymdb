use home::home_dir;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub db_folder: PathBuf,
    pub db_file: PathBuf,
}

fn make_path(dest: &str) -> PathBuf {
    let mut path = home_dir().unwrap();
    let d = PathBuf::from(dest);
    path.push(d);
    path
}

impl Config {
    pub fn init() -> Config {
        let db_folder = make_path(".config/ymdb");
        let db_file = make_path(".config/ymdb/ymdb.db");
        Config { db_folder, db_file }
    }

    pub fn verify_config(&self) {
        self.check_or_create_config_folder();
        self.check_or_create_database_file();
    }

    fn check_or_create_config_folder(&self) {
        if !self.db_folder.exists() {
            create_dir_all(self.db_folder.as_path()).unwrap();
        }
    }

    fn check_or_create_database_file(&self) {
        if !self.db_file.exists() {
            let _ = File::create(self.db_file.as_path());
        }
    }
}
