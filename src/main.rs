use std::{env::args, process::exit};

use application::Application;
use colored::Colorize;
use enums::arg::Arg;
use utils::logger::Logger;

mod application;
mod enums;
mod files;
mod utils;

fn main() {
    let arg = Arg::from_str(&args().nth(1).unwrap_or_default());

    if let Arg::None = arg {
        eprintln!("Invalid argument. Use 'help' to see a list of valid arguments.");
        exit(1);
    }

    let app = Application::new();
    if let Err(e) = app {
        eprintln!(
            "{} {}",
            "Failed to create application:\n\t".bright_red(),
            e.to_string().red()
        );
        return;
    }

    let mut app = app.unwrap();

    match arg {
        Arg::Help => Application::display_help(),
        Arg::Add => {
            let (original_path, symlink_path) = get_link_args();
            app.create_managed_link(&original_path, symlink_path);
        }
        _ => {}
    }
}

fn get_link_args() -> (String, String) {
    let original_path = args().nth(2).unwrap_or_default();
    if original_path.is_empty() {
        eprintln!("Invalid arguments. Use 'help' to see a list of valid arguments.");
        exit(1);
    }

    // Determine if the path is valid and if absolute or relative
    let original_path_buff = std::path::Path::new(&original_path)
        .canonicalize()
        .unwrap_or_else(|_| {
            eprintln!("Path does not exist or is invalid: {}", original_path);
            exit(1);
        });

    if original_path_buff.is_symlink() {
        eprintln!("Unable to create a symlink to a symlink: {}", original_path);
        exit(1);
    }

    let mut original_path = original_path_buff.to_str().unwrap().to_string();
    if original_path.starts_with("\\\\?\\") {
        original_path = original_path[4..].to_string();
    }

    let symlink_path = args().nth(3).unwrap_or_default();

    (original_path, symlink_path)
}
