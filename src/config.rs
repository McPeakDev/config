//Config
use serde::de::DeserializeOwned;
use std::{env, fs::File};

pub fn load_config<T: DeserializeOwned>(file_path: &str) -> Option<T> {
    let file_result = File::open(file_path);

    if file_result.is_ok() {
        let file = file_result.unwrap();
        let reader_result = serde_json::from_reader(file);

        if reader_result.is_ok() {
            let config: T = reader_result.unwrap();
            return Some(config);
        } else {
            println!("Format Error: {} is not well formatted JSON", file_path);
        }
    } else {
        let path_result = env::current_dir();

        println!("File Error: {} is not found or does not exist.", file_path);

        if path_result.is_ok() {
            let path = format!("{}", path_result.unwrap().display());
            let mut slash = "\\";

            if env::consts::OS == "linux" {
                slash = "/"
            }

            println!(
                "The file should be placed here: {}{}{}",
                path, slash, file_path
            );
        }
    }

    return None;
}
