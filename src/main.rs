mod command;
mod database;

use command::{Command, parse_command_args, parse_command_type};
use database::Database;
use std::{
    io::{self, Write},
    process::exit,
    sync::{Arc, RwLock},
};

fn main() {
    let db_path = "rdb.json";
    let db = RwLock::new(Database::load_from_file(db_path));
    let db = Arc::new(db);

    let mut input = String::new();

    loop {
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read command");

        let command: Vec<&str> = input.split_whitespace().collect();

        if Option::is_none(&command.first()) {
            eprintln!("Command is empty")
        }

        let command_type = parse_command_type(command.first().unwrap());
        let command_args = parse_command_args(input.trim());

        match command_type {
            Command::Insert if command_args.len() == 2 => {
                let mut db_lock = db.write().unwrap();
                db_lock.insert(command_args[0], command_args[1]);
            }
            Command::Insert => println!("Usage: insert <key> <value>"),
            Command::Delete if command_args.len() == 1 => {
                let mut db_lock = db.write().unwrap();
                db_lock.delete(command_args[0]);
            }
            Command::Delete => println!("Usage: delete <key>"),
            Command::Update if command_args.len() == 2 => {
                let mut db_lock = db.write().unwrap();
                db_lock.update(command_args[0], command_args[1]);
            }
            Command::Update => println!("Usage: update <key> <value>"),
            Command::Get if command_args.len() == 1 => {
                let mut db_lock = db.write().unwrap();
                db_lock.get(command_args[0]);
            }
            Command::Get => println!("Usage: get <key>"),
            Command::Exit => {
                println!("Saving datata");
                let db_lock = db.write().unwrap();
                db_lock.save_to_file(db_path);

                exit(1)
            }
            Command::Unknown => continue,
        }

        input = String::new();
    }
}
