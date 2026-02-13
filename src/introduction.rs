const RESET: &str = "\x1b[0m";
const YELLOW: &str = "\x1b[33m";

pub fn print_introduction() {
    println!("{}RSList{} - Simple ToDo CLI in Rust\n", YELLOW, RESET);
}

pub fn print_usage() {
    println!("{}Usage{}", YELLOW, RESET);
    println!("- rslist <section> <command>\n");
    println!("{}Commands{}", YELLOW, RESET);
    println!("- test\t\tRun a test command");
    println!("- config\tShow configuration\n");
}
