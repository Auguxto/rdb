# RDB - Rust In-Memory Database with Persistence

## Overview

RDB is a simple in-memory key-value database written in Rust that provides persistence through file storage. It offers basic CRUD operations (Create, Read, Update, Delete) and maintains data between program executions by saving to a JSON file.

## Features

- **In-memory storage**: Fast data access using Rust's HashMap
- **Persistence**: Data is saved to disk in JSON format
- **Simple CLI interface**: Easy-to-use command-line interface

## Commands

RDB supports the following commands:

| Command | Usage | Description |
|---------|-------|-------------|
| `insert` | `insert <key> <value>` | Adds a new key-value pair to the database |
| `get` | `get <key>` | Retrieves the value associated with the specified key |
| `update` | `update <key> <value>` | Updates the value of an existing key |
| `delete` | `delete <key>` | Removes a key-value pair from the database |
| `exit` | `exit` | Saves the database to disk and exits the program |

## Implementation Details

### Database Structure

The database is implemented as a struct containing a HashMap that maps string keys to string values. The struct is serializable and deserializable using serde, which enables persistence.

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub data: HashMap<String, String>,
}
```

### Persistence

Data persistence is achieved by serializing the database to JSON and writing it to a file. When the program starts, it attempts to load the database from the file. If the file doesn't exist or is corrupted, a new empty database is created.

```rust
pub fn load_from_file(path: &str) -> Self {
    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Database::new()),
        Err(_) => Database::new(),
    }
}

pub fn save_to_file(&self, path: &str) {
    if let Ok(json) = serde_json::to_string_pretty(&self) {
        let _ = fs::write(path, json);
    }
}
```

### Command Processing

Commands are parsed from user input and processed using pattern matching. Each command is mapped to a corresponding method in the Database struct.

## Building and Running

```bash
# Clone the repository
git clone https://github.com/yourusername/rdb.git
cd rdb

# Build the project
cargo build --release

# Run the database
cargo run --release
```

## Example Usage

```
$ cargo run
insert name John
get name
"John"
update name Jane
get name
"Jane"
delete name
exit
Saving data
```
