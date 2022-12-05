use clap::{Parser, Subcommand};
use cli_grocery_list::{add_item, check_if_item_exists, check_source_exists, read_csv_data, delete_item};
use std::char::ToLowercase;

#[derive(Parser)]
#[command(name = "CLI Grocery List")]
#[command(author = "Josh M. <joshua.mo.876@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "A grocery list with full CRUD functionality and output to xlsx, JSON or a pdf.", long_about = None)]

struct Cli {
    #[arg(long)]
    item: Option<String>,
    #[arg(long)]
    one: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Find {
        item: String,
    },
    Add {
        quantity: i32,
        item: String,
    },
    Delete {
        item: String,
    }
}

fn main() {
    check_source_exists().ok();
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => {
            let meme = read_csv_data().unwrap();
            println!("You currently have the following on your grocery list:");
            for item in meme {
                println!("- {}x {}", item.quantity, item.name)
            }
        }
        Commands::Find { item } => {
            if check_if_item_exists(item.to_owned()) == true {
                println!("{} is on the grocery list!", item)
            } else {
                println!("This item isn't on the grocery list...")
            }
        }
        Commands::Add { quantity, item } => {
            add_item(quantity, item);
        }
        Commands::Delete { item } => {
            delete_item(item.to_owned());
            println!("You removed: {:?}", item)
        }
    }
}
