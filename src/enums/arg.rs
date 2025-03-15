pub enum Arg {
    None,
    Add,
    Remove,
    List,
    Help,
}

impl Arg {
    pub fn from_str(s: &str) -> Arg {
        match s {
            "a" | "add" => Arg::Add,
            "r" | "remove" => Arg::Remove,
            "l" | "list" => Arg::List,
            "h" | "help" => Arg::Help,
            _ => Arg::None,
        }
    }
}
