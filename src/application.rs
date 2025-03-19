use crate::files::{config::Config, managed_link::ManagedLink, managed_link_list::ManagedLinkList};
use crate::utils::file_utils::FileUtils;
use serde_json::Error;

pub struct Application {
    pub managed_link_list: ManagedLinkList,
    pub config: Config,
    file_utils: FileUtils,
}

impl Application {
    pub fn new() -> Result<Application, Error> {
        let file_utils = FileUtils::new();
        let config_res = Config::load(&file_utils);

        if let Ok(config) = config_res {
            let managed_link_list = ManagedLinkList::load(&file_utils);
            return Ok(Application {
                managed_link_list,
                config,
                file_utils,
            });
        }

        Err(config_res.err().unwrap())
    }

    pub fn display_help() {
        println!("Valid arguments:");
        println!("\t- add <original_path> [symlink_path]");
        println!("\t\tCreates a symbolic link from the original path to the symlink path");
        println!("\t\tIf no symlink path is provided, the default symlink directory is used");
    }

    pub fn create_managed_link(&mut self, original_path: &String, mut target: String) {
        if target.is_empty() {
            target.push_str(&self.config.get_symlink_dir());
        }

        // Symlinks follow the structure of the original path to ease navigation
        target.push_str("\\");
        target.push_str(&original_path.replace(":", ""));

        let time = chrono::Local::now().to_string();

        let link = ManagedLink {
            id: self.managed_link_list.get_last_id() + 1,
            original_path: original_path.clone(),
            symlink_path: target.clone(),
            creation_date: time.clone(),
            last_modified: time.clone(),
        };

        println!(
            "About to create symbolic link:\n\n{}\n\nContinue? [y/N]",
            link
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() != "y" {
            println!("Aborted.");
            return;
        }

        self.managed_link_list.managed_links.push(link);
        self.managed_link_list.save(&self.file_utils);

        println!("Link created successfully.");
    }
}
