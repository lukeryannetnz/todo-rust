extern crate clap;
use clap::{Arg, App};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, Read};
extern crate serde;
extern crate serde_json;

static FILEPATH: &'static str = "./todo.txt";

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
            print_items(&items);
        }
        "edit" => {
            print_items(&items);
            println!("which item would you like to edit? Enter the index number:");
            let itemindex = select_item(&items) - 1;
            update_item(&mut items, itemindex);
            write_items(items);
        }
        "delete" => {
            print_items(&items);
            println!("which item would you like to delete? Enter the index number:");
            let itemindex = select_item(&items) - 1;
            delete_item(&mut items, itemindex);
            write_items(items);
        }
        _ => println!("unrecognised command"),
    }
}

fn delete_item(items: &mut Vec<String>, itemindex: usize) {
    items.remove(itemindex);
}

fn update_item(items: &mut Vec<String>, itemindex: usize) {
    println!("please enter a new todo item");
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect(
        "Failed to read line",
    );

    items[itemindex] = buffer;
}

fn select_item(items: &Vec<String>) -> usize {
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect(
            "Failed to read line",
        );

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        if guess < 1 || guess > items.len() {
            println!("Please enter the index of one of the items.");
            continue;
        }

        return guess;
    }
}

fn print_items(items: &Vec<String>) {
    println!("here are all of your items to do:");

    let mut x = 1;
    for item in items {
        println!("{} {}", x, item);
        x += 1;
    }
}

fn load_items(mut items: Vec<String>) -> Vec<String> {
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

fn write_items(items: Vec<String>) {
    let json = serde_json::to_string(&items).unwrap();

    let mut file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(FILEPATH)
        .unwrap();

    file.write_all(json.as_bytes()).unwrap();
}

fn configure_app<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("todo")
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
}
