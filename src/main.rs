pub mod commands;
pub mod errors;

use std::env;

use crate::commands::Commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Commands::from(&args[2].to_string());

    match command {
        Commands::Test => println!("Test Command"),
        _ => panic!("Invalid command"),
    }
}
