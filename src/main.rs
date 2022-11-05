#![allow(unused_variables)]
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::io::{stdin, Error, ErrorKind};

#[derive(Parser)]
pub struct Pos {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Setup,

    Transaction { product_name: String, amount: i32 },
}
fn main() -> Result<(), std::io::Error> {
    let mut items: HashMap<&str, i32> = HashMap::new();
    let mut item_name = String::new();
    let mut item_amount = String::new();
    let args = Pos::parse();
    match args.command {
        Commands::Setup => {
            println!("Enter the item name: ");
            stdin()
                .read_line(&mut item_name)
                .expect("Please enter valid item name");
            println!("Enter the amount of {item_name} you have in your inventory: ");
            stdin()
                .read_line(&mut item_amount)
                .expect("Please enter valid number");
            items.insert(&item_name, item_amount.parse::<i32>().unwrap());
        }
        Commands::Transaction {
            product_name,
            amount,
        } => {
            if let Some(item) = items.get_mut(product_name.as_str()) {
                if *item == 0 {
                    println!("No {item} left in inventory");
                    return Err(Error::new(ErrorKind::NotFound, "Empty"));
                }
                *item -= amount;
                println!("{} {} left", *item, product_name);
            }
        }
    }
    Ok(())
}
