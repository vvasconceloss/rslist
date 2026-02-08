use std::io::{self, Write};

fn main() {
    io::stdout()
        .flush()
        .expect("An error occurred while obtaining user input");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("An error occurred while reading the user input");

    print!("{}", user_input);
}
