extern crate clap;
use clap::{Arg, App};
use std::fs::File;
use std::io::prelude::*;
extern crate serde;
extern crate serde_json;

fn main() {
    println!("Hello, world!");
    let mut items: Vec<String> = Vec::new();

    let matches = App::new("todo")
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
        )
        .get_matches();

    let command = matches.value_of("command").unwrap_or("list");

    let mut file = File::create("todo.txt").unwrap();

    match command {
        "new" => {
            let item = matches.value_of("item").unwrap_or("").to_string();
            println!("creating a new todo item. {}", item);
            items.push(item);
            let json = serde_json::to_string(&items).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
        "list" => println!("listing todo items"),
        "edit" => println!("editing todo items"),
        "delete" => println!("removing a todo item"),
        _ => println!("unrecognised command"),
    }
}
