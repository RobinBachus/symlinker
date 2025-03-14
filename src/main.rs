use colored::Colorize;
use application::Application;
use files::managed_link;

mod application;
mod files {
    pub mod managed_link;
    pub mod managed_link_list;
    pub mod config;
}
mod utils {
    pub mod file_utils;
}

fn main() {
    let app = Application::new();
    if let Err(e) = app {
        println!("{} {}", "Failed to create application:\n\t".bright_red(), e.to_string().red());
        return;
    }

    let mut app = app.unwrap();

    app.display_managed_links();
    app.display_config();

    let link = managed_link::ManagedLink {
        id: 0,
        original_path: "".to_string(),
        symlink_path: "".to_string(),
        creation_date: "".to_string(),
        last_modified: "".to_string(),
        link_type: "".to_string(),
    };

    app.add_managed_link(link);
    app.display_managed_links();
}
