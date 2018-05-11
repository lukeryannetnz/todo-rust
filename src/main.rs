extern crate clap;
extern crate serde;
extern crate serde_json;

mod filesystem;

use clap::{Arg, App};
use std::io::{self};

type Persistence = filesystem::FileSystem;

pub trait Load {
    fn load() -> Vec<String>;
}

pub trait Write {
    fn write(Vec<String>);
}

fn main() {
    println!("Welcome to the command line todo manager, written in rust");

    let app = configure_app();
    let matches = app.get_matches();
    let command = matches.value_of("command").unwrap_or("list");

    let mut items = Persistence::load();

    match command {
        "new" => {
            let item = matches.value_of("item").unwrap_or("").to_string();
            println!("creating a new todo item. {}", item);
            items.push(item);
            Persistence::write(items);
        }
        "list" => {
            print_items(&items);
        }
        "edit" => {
            print_items(&items);
            println!("which item would you like to edit? Enter the index number:");
            let itemindex = select_item(&items) - 1;
            update_item(&mut items, itemindex);
            Persistence::write(items);
        }
        "delete" => {
            print_items(&items);
            println!("which item would you like to delete? Enter the index number:");
            let itemindex = select_item(&items) - 1;
            delete_item(&mut items, itemindex);
            Persistence::write(items);
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
