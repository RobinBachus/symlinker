use crate::files::config::Config;
use crate::files::managed_link::ManagedLink;
use crate::files::managed_link_list::ManagedLinkList;
use crate::utils::file_utils::FileUtils;
use crate::utils::logger::Logger;
use colored::Colorize;
use serde_json::Error;
use std::path::PathBuf;

pub struct Application {
    pub managed_link_list: ManagedLinkList,
    pub config: Config,
    file_utils: FileUtils,
}

impl Application {
    pub fn new() -> Result<Application, Error> {
        let file_utils = FileUtils::new();
        let config_res = Config::load(&file_utils);

        Logger::clear_or_create_file(&file_utils).unwrap();

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

        let files = FileUtils::get_files_in_dir(&PathBuf::from(original_path), 0).unwrap();

        let sl_path = link.symlink_path.as_str();
        let disk_name = sl_path.split_at(sl_path.find('\\').unwrap() + 1).0;
        let disk_sizes = FileUtils::get_disk_size_info(&disk_name).unwrap();
        let available_size = disk_sizes.0;
        let total_size = disk_sizes.1;

        println!(
            "About to create symbolic link:\n\n{}\n\nThis will move {} files.\nThe symlink drive has {} of {} space left.\n\nList files to move? [y/N]",
            link,
            files.len(),
            FileUtils::bytes_to_human_readable(available_size).cyan(),
            FileUtils::bytes_to_human_readable(total_size).blue()
        );

        let link_formatted = link.to_string().replace("\n", "\n\t");
        Logger::log_to_file(
            &self.file_utils,
            &format!("About to create link:\n\t{link_formatted}"),
        )
        .unwrap();

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() == "y" {
            println!("\nThese files will be moved:\n");

            for file in files {
                println!(
                    "{}",
                    FileUtils::path_to_relative(&file, &original_path).blue()
                );
            }
        }

        println!("\nCreate link? [y/N]");
        input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() != "y" {
            println!("Aborted.");
            Logger::log_to_file(&self.file_utils, "Link creation aborted").unwrap();
            return;
        }

        let link_id = link.id.clone();
        self.managed_link_list.managed_links.push(link);
        self.managed_link_list.save(&self.file_utils);

        Logger::log_to_file(
            &self.file_utils,
            &format!("Created link with ID {}", link_id),
        )
        .unwrap();

        println!("Link created successfully.");
    }
}
