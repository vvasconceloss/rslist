pub mod commands;
pub mod errors;
pub mod introduction;

use std::env;

use crate::introduction::print_usage;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        return;
    }

    let command_line = &args[2];

    match command_line.as_str() {
        "test" => println!("Test Command"),
        "config" => println!("Config Command"),
        other_command => {
            print_usage();
            println!("Unknown command: {}", other_command);
        }
    }
}
