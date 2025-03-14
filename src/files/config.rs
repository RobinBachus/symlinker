use std::fmt::Display;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde::ser::Error as e;
use crate::utils::file_utils::{FileUtils, ProjectDirType};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub create_backup: bool,
    pub backup_dir: PathBuf,
    pub default_symlink_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            create_backup: true,
            backup_dir: PathBuf::from("C:\\symlinker_backups"),
            default_symlink_dir: PathBuf::from("C:\\symlinks"),
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}", serde_json::to_string_pretty(self).unwrap()
        )
    }
}

impl Config {
    pub fn load(file_utils: &FileUtils) -> Result<Config, serde_json::error::Error> {
        let res = file_utils.parse_file(ProjectDirType::Config, "config.json");

        if let Ok(config) = res {
            return Ok(config);
        }

        println!("Failed to load config file. Would you like to create a new config file? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "y" {
            let config = Config::default();
            config.save(file_utils);
            return Ok(config);
        }

        Err(serde_json::error::Error::custom(format!("Invalid config file: {}" , res.err().unwrap())))
    }

    pub fn save(&self, file_utils: &FileUtils) {
        file_utils.save_file(ProjectDirType::Config, "config.json", self);
    }
}
