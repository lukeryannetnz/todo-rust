use std::fs::OpenOptions;
use std::io::{Read};
use std::io::prelude::*;
use Load;
use Write as TodoWrite;

extern crate serde;
extern crate serde_json;

static FILEPATH: &'static str = "./todo.txt";
pub struct FileSystem;

impl Load for FileSystem{
    fn load() -> Vec<String> {
        let mut items: Vec<String> = Vec::new();
        let mut buffer = String::new();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(FILEPATH)
            .unwrap();

        file.read_to_string(&mut buffer).unwrap();

        if buffer.len() > 0 {
            items = serde_json::from_str::<Vec<String>>(&buffer).unwrap();
        }

        items
    }
}

impl TodoWrite for FileSystem{
    fn write(items: Vec<String>) {
        let json = serde_json::to_string(&items).unwrap();

        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(FILEPATH)
            .unwrap();

        file.write_all(json.as_bytes()).unwrap();
    }
}