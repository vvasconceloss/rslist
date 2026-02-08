#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    Test,
    Config,
}

impl From<&String> for Commands {
    fn from(value: &String) -> Self {
        let value_str = value.as_str();

        match value_str {
            "test" => Commands::Test,
            _ => panic!("Invalid command"),
        }
    }
}
