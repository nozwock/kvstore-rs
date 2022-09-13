use colored::*;
use std::collections::HashMap;
use std::fs::read_to_string;

const DB_NAME: &str = "kvstore.db";
const KVSTORE_DELIMITER: char = '=';

fn main() {
    const valid_cmds: (&str, &str) = ("set", "get");

    // accessing the args
    let mut cli_args = std::env::args().skip(1);
    let key = cli_args.next().unwrap();
    let value = cli_args.next().unwrap();

    println!("{:?}\n{:?}", cli_args, key);

    // intializing the database
    let kvstore = Database::new(DB_NAME.to_owned()).unwrap();

    // chunk to write (set case)
    let chunk = format!("{}{KVSTORE_DELIMITER}{}\n", key, value);
}

// A simple UTF-8 kv database.
struct Database {
    map: HashMap<String, String>,
    db: String,
}

impl Database {
    fn new(db: String) -> Result<Database, std::io::Error> {
        // new(db: String) fn returns a Database instance populated with
        // the entries from the database file.
        let mut kvstore = Database {
            map: HashMap::new(),
            db,
        };
        // umm, currently does not create the db file if not available...hehe
        let content = read_to_string(&kvstore.db)?;
        for line in content.lines() {
            let (key, value) = line.split_once(KVSTORE_DELIMITER).unwrap();
            // populating the database
            kvstore
                .map
                .insert(key.to_owned(), value.to_owned())
                .unwrap();
        }
        Ok(kvstore)
    }
    // to insert a kv pair in the database
    fn insert() {}
    // to fetch the value for the given key from the database
    fn get() {}
}
