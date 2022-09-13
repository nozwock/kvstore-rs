use colored::*;
use std::collections::HashMap;
use std::fs::{read_to_string, write};

const DB_NAME: &str = "kvstore.db";
const KVSTORE_DELIMITER: &str = " -> ";

fn main() {
    const valid_cmds: (&str, &str) = ("set", "get");

    // accessing the args
    let mut cli_args = std::env::args().skip(1);
    println!("{:?}", cli_args);
    let key = cli_args.next().unwrap();
    let value = cli_args.next().unwrap();

    println!(
        "{}{KVSTORE_DELIMITER}{}",
        key.bright_yellow(),
        value.bright_cyan()
    );

    // intializing the database
    let mut kvstore = Database::new(DB_NAME).unwrap();
    kvstore.insert(key, value);
    // writes to the file and rids of the database
    kvstore.flush().unwrap();
}

// A simple UTF-8 kv database.
struct Database {
    map: HashMap<String, String>,
    db: String,
}

impl Database {
    // new(db: String) associated fn returns a Database instance populated with
    // the entries from the database file.
    fn new(db: &str) -> Result<Database, std::io::Error> {
        let mut kvstore = Database {
            map: HashMap::new(),
            db: db.to_owned(),
        };
        // umm, currently does not create the db file if not available...hehe
        let contents = read_to_string(&kvstore.db)?;
        for line in contents.lines() {
            let (key, value) = line.split_once(KVSTORE_DELIMITER).unwrap();
            // populating the database
            kvstore
                .map
                .insert(key.to_owned(), value.to_owned())
                .unwrap();
        }
        Ok(kvstore)
    }
    // insert() method to insert a kv pair in the database
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    // Flushes the database data to the disk file
    // we're still taking ownership here intentionally,
    // throw the db away after contentss has been written to disk file.
    // ------here
    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            let chunk = format!("{}{KVSTORE_DELIMITER}{}\n", key, value);
            contents.push_str(&chunk);
        }
        write(self.db, contents)
    }

    // to fetch the value for the given key from the database
    fn get() {
        todo!();
    }
}
