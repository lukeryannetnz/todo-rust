extern crate clap;
use clap::{Arg, App};
use std::fs::OpenOptions;
use std::io::prelude::*;
extern crate serde;
extern crate serde_json;

fn main() {
    println!("Welcome to the command line todo manager, written in rust");

    let app = configure_app();
    let matches = app.get_matches();
    let command = matches.value_of("command").unwrap_or("list");

    let mut items: Vec<String> = Vec::new();
    items = load_items(items);

    match command {
        "new" => {
            let item = matches.value_of("item").unwrap_or("").to_string();
            println!("creating a new todo item. {}", item);
            items.push(item);

            write_items(items);
        }
        "list" => {
            println!("here are all of your items to do:");

            for item in items {
                println!("{}", item);
            }
        }
        "edit" => println!("editing todo items"),
        "delete" => println!("removing a todo item"),
        _ => println!("unrecognised command"),
    }
}

fn load_items(mut items: Vec<String>) -> Vec<String> {
    let mut buffer = String::new();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todo.txt")
        .unwrap();

    file.read_to_string(&mut buffer).unwrap();

    if buffer.len() > 0 {
        items = serde_json::from_str::<Vec<String>>(&buffer).unwrap();
    }

    items
}

fn write_items(items: Vec<String>) {
    let mut file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open("todo.txt")
        .unwrap();

    let json = serde_json::to_string(&items).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn configure_app<'a, 'b>() -> clap::App<'a, 'b> {
    let app = App::new("todo")
        .version("1.0")
        .author("Luke <luke.ryan@xero.com>")
        .about("Manages todos")
        .arg(
            Arg::with_name("command")
                .short("c")
                .long("command")
                .value_name("command")
                .help(
                    "Specifies the command to run. Must be new, list, edit or delete",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("item")
                .short("i")
                .long("item")
                .value_name("item")
                .help(
                    "Contains the todo item text. Should be used with new and edit commands.",
                )
                .takes_value(true),
        );
    app
}
