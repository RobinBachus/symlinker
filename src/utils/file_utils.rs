use crate::enums::project_dir_type::ProjectDirType;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

pub(crate) struct FileUtils {
    project_dirs: ProjectDirs,
}

impl FileUtils {
    pub(crate) fn new() -> FileUtils {
        let project_dirs = ProjectDirs::from("xyz", "server1rb", "symlinker").unwrap();

        FileUtils { project_dirs }
    }

    /// Parses a file and returns the data as a Result<T, Error>
    /// # Arguments
    /// * `dir_type`: What directory to look in
    /// * `file_name`: The name of the file to parse (including extension)
    /// # Returns
    /// * `Result<T, Error>`: The data from the file
    pub(crate) fn parse_file<T: Default + Serialize + for<'a> Deserialize<'a>>(
        &self,
        dir_type: ProjectDirType,
        file_name: &str,
    ) -> Result<T, serde_json::Error> {
        let dir = match dir_type {
            ProjectDirType::Data => self.project_dirs.data_dir(),
            ProjectDirType::Config => self.project_dirs.config_dir(),
        };

        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join(file_name);
        if !file_path.exists() {
            println!("Creating file: {}", file_path.display());
            let default: T = Default::default();
            std::fs::write(&file_path, serde_json::to_string(&default)?).unwrap();
        }

        let str = &std::fs::read_to_string(&file_path).unwrap();
        serde_json::from_str(str)
    }

    /// Saves a file with the given data
    /// # Arguments
    /// * `dir_type`: What directory to save the file in
    /// * `file_name`: The name of the file to save (including extension)
    /// * `data`: The data to save
    pub(crate) fn save_file<T: Serialize>(
        &self,
        dir_type: ProjectDirType,
        file_name: &str,
        data: &T,
    ) {
        let dir = match dir_type {
            ProjectDirType::Data => self.project_dirs.data_dir(),
            ProjectDirType::Config => self.project_dirs.config_dir(),
        };

        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join(file_name);
        std::fs::write(&file_path, serde_json::to_string(&data).unwrap()).unwrap();
    }
}
