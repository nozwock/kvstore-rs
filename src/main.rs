use clap::{Parser, Subcommand};
use colored::*;
use lib::kv_store::*;

pub mod lib;

const DB_PATH: &str = "kvstore.db";

#[derive(Parser)]
#[clap(version)]
#[clap(about = "Simple CLI program which stores key-value pairs in a file")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// add the key-value pair to the database
    Add {
        #[clap(value_parser)]
        key: String,

        #[clap(value_parser)]
        value: String,
    },
    /// return the value corresponding to the given key
    Get {
        #[clap(value_parser)]
        key: String,
    },
    /// lists all key-value pairs in the database
    List,
    /// remove the key-value pair from the database
    Remove {
        #[clap(value_parser)]
        key: String,
    },
}

fn main() {
    // // accessing the args
    // let mut cli_args = std::env::args().skip(1);
    // println!("{:?}", cli_args);
    // let key = cli_args.next().unwrap();
    // let value = cli_args.next().unwrap();

    // using clap
    let cli = Cli::parse();

    // intializing the database
    let mut kvstore = Database::new(DB_PATH).expect("Failed to initialize the database");

    match &cli.command {
        Commands::Add { key, value } => {
            kvstore.insert(key.to_string(), value.to_string());
            println!(
                "{}\n{}",
                "=> Adding to database".bright_yellow(),
                format!("+ `{}{KVSTORE_DELIMITER}{}`", key, value).bright_green()
            );
        }
        Commands::Get { key } => match kvstore.get(key.to_string()) {
            Some(value) => println!("{}", value),
            None => println!("{}", "✗ Key does not exists!".bright_red()),
        },
        Commands::List => {
            println!("{:#?}", kvstore.map);
        }
        Commands::Remove { key } => match kvstore.remove(key) {
            Some(value) => println!(
                "{}\n{}",
                "=> Removing from database".bright_yellow(),
                format!("- `{}{KVSTORE_DELIMITER}{}`", key, value).bright_red()
            ),
            None => println!("{}", "✗ Key does not exists!".bright_red()),
        },
    }
    kvstore.flush().unwrap();
}
