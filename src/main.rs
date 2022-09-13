use colored::*;
use lib::kv_store::*;
use rustyline::Editor;

pub mod lib;

const DB_PATH: &str = "kvstore.db";

fn main() -> rustyline::Result<()> {
    // accessing the args
    // let mut cli_args = std::env::args().skip(1);
    // println!("{:?}", cli_args);
    // let key = cli_args.next().unwrap();
    // let value = cli_args.next().unwrap();

    // intializing the database
    let mut kvstore = Database::new(DB_PATH).expect("Failed to initialize the database");

    // using rustyline
    let mut rl = Editor::<()>::new()?;
    loop {
        let line = rl.readline(">> ")?;
        if line.as_str() == "quit" {
            kvstore.flush().unwrap();
            break;
        }

        let mut chunks = line.split_ascii_whitespace();
        let command = &chunks.next().unwrap();
        let key = &chunks.next().unwrap();
        let value = &chunks.next().unwrap();

        println!(
            "{}{KVSTORE_DELIMITER}{}",
            key.bright_yellow(),
            value.bright_cyan()
        );
        kvstore.insert(key.to_string(), value.to_string());
    }
    Ok(())
}
