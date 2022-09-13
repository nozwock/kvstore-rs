use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    const valid_cmds: (&str, &str) = ("set", "get");

    let mut cli_args = std::env::args().skip(1);
    let key = cli_args.next().unwrap();
    let value = cli_args.next().unwrap();

    println!("{:?}\n{:?}", cli_args, key);

    let content = format!("{}={}\n", key, value);
}

struct Database {
    map: HashMap<String, String>,
    db: String,
}

impl Database {
    fn new(db_path: String) -> Database {
        Database {
            map: HashMap::new(),
            db: db_path,
        };
        let contents = read_to_string(db_path).unwrap();
    }
    fn write() {}
    fn read() {}
}
