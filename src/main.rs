mod utils;

use clap::{Parser, Subcommand};
use cli_grocery_list::{add_item, check_if_item_exists, delete_item, update_item};
use utils::lib::{read_csv_data, init_data, Info};

#[derive(Parser)]
#[command(name = "CLI Grocery List")]
#[command(author = "Josh M. <joshua.mo.876@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "A CLI shopping list with full CRUD functionality and output to xlsx, JSON or a pdf.", long_about = None)]

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
    /// List all items
    List,
    /// Find one item
    Find {
        item: String,
    },
    /// Add an item
    Add {
        quantity: i32,
        item: String,
        price: f32
    },
    /// Delete an item from the list
    Delete {
        item: String,
    },
    /// Update an item in the list
    Update {
        item: String,
        new_quantity: i32,
    },
    /// Sort items in the list
    Sort
}

fn main() {
    let cli = Cli::parse();
    println!("---");
    match &cli.command {
        Commands::List => {
            println!("You currently have the following on your shopping list:");
            for item in read_csv_data().unwrap() {
                if item.quantity.parse::<i32>().unwrap() > 1 {
                    println!("- {}x {} @ £{} each", item.quantity, item.name, item.price);
                } else {
                    println!("- {}x {} @ £{}", item.quantity, item.name, item.price);
                };
            }
            println!("{}", init_data().get_total_qty());
            println!("{}", init_data().get_total_cost());
        }
        Commands::Find { item } => {
            if check_if_item_exists(item.to_owned()) == true {
                println!("{} is on the shopping list!", item)
            } else {
                println!("This item isn't on the shopping list...")
            }
        }
        Commands::Add { quantity, item, price} => {
            add_item(quantity, item, *price).ok();
        }
        Commands::Delete { item } => {
            delete_item(item.to_owned()).map_err(|e| e.to_string()).ok();
            println!("You removed: {}", item)
        },
        Commands::Update {item, new_quantity} => {
            update_item(item.to_owned(), *new_quantity);
            println!("You updated {item} to be {new_quantity}.")
        },
        Commands::Sort => {
            init_data().sort();
        }
    }
    println!("---");
}
