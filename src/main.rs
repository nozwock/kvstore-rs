use clap::{Parser, Subcommand};
use lib::kv_store::*;

pub mod lib;

const DB_PATH: &str = "kvstore.db";

#[derive(Parser)]
#[clap(version)]
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

    // intializing the database
    let mut kvstore = Database::new(DB_PATH).expect("Failed to initialize the database");

    // using clap
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { key, value } => {
            // println!("DEBUG\n-----\n{}{KVSTORE_DELIMITER}{}", key, value,);

            kvstore.insert(key.to_string(), value.to_string());
        }
        Commands::Get { key } => match kvstore.get(key.to_string()) {
            Some(value) => println!("{}", value),
            None => println!("Key does not exists!"),
        },
        Commands::List => {
            println!("{:#?}", kvstore.map);
        }
        Commands::Remove { key } => match kvstore.remove(key) {
            Some(value) => println!("Successfully removed {}{KVSTORE_DELIMITER}{}", key, value),
            None => println!("Key does not exists!"),
        },
    }
    kvstore.flush().unwrap();
}
