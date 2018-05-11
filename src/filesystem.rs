use std::fs::OpenOptions;
use std::io::{Read};

extern crate serde;
extern crate serde_json;

static FILEPATH: &'static str = "./todo.txt";

pub fn load_items(mut items: Vec<String>) -> Vec<String> {
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