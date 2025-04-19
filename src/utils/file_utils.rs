use crate::enums::project_dir_type::ProjectDirType;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    io::Error,
    path::{Path, PathBuf},
};
use sysinfo::Disks;

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
        let file_path = self.resolve_file_path(dir_type, file_name);

        FileUtils::create_file_if_not_exist::<T>(&file_path)?;

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
    ) -> Result<(), Error> {
        let file_path = self.resolve_file_path(dir_type, file_name);
        std::fs::write(&file_path, serde_json::to_string(&data).unwrap())?;
        return Ok(());
    }

    /// Appends data to a file. If the file does not exist, it will be created.
    ///
    /// # Arguments
    /// * `dir_type` - The type of directory where the file is located (e.g., data or config).
    /// * `file_name` - The name of the file to append data to (including extension).
    /// * `data` - The string data to append to the file.
    ///
    /// # Returns
    /// * `Result<(), Error>` - Returns `Ok(())` if the operation is successful, or an error if it fails.
    ///
    /// # Behavior
    /// * If the file does not exist, it will be created with default content before appending the data.
    /// * The appended data will be serialized as JSON and written to the file.
    ///
    /// # Errors
    /// * Returns an error if the file cannot be read, written, or created.
    pub(crate) fn append(
        &self,
        dir_type: ProjectDirType,
        file_name: &str,
        data: &str,
    ) -> Result<(), Error> {
        let file_path: PathBuf = self.resolve_file_path(dir_type, file_name);
        FileUtils::create_file_if_not_exist::<String>(&file_path)?;

        let mut content = std::fs::read_to_string(&file_path).unwrap();

        content.insert_str(content.len(), data);
        std::fs::write(&file_path, content)?;

        Ok(())
    }

    pub(crate) fn write(
        &self,
        dir_type: ProjectDirType,
        file_name: &str,
        data: &str,
    ) -> Result<(), Error> {
        let file_path: PathBuf = self.resolve_file_path(dir_type, file_name);
        FileUtils::create_file_if_not_exist::<String>(&file_path)?;

        std::fs::write(&file_path, data)?;
        Ok(())
    }

    pub(crate) fn get_disk_size_info(disk_mount: &str) -> Result<(u64, u64), String> {
        let mut disks = Disks::new();
        disks.refresh(true);

        let disk = disks
            .list()
            .iter()
            .find(|d| d.mount_point() == Path::new(disk_mount));
        if disk.is_none() {
            return Err(format!("Disk '{}' not found", disk_mount));
        }

        Ok((disk.unwrap().available_space(), disk.unwrap().total_space()))
    }

    pub(crate) fn bytes_to_human_readable(size: u64) -> String {
        let units = ["b", "kb", "mb", "gb", "tb"];

        let mut m_size: u64 = size;

        let base: u64 = 1000;
        for (i, unit) in units.iter().enumerate() {
            m_size = size / base.pow(i.try_into().unwrap());
            if m_size < 1000 {
                return format!("{m_size}{unit}");
            }
        }

        return format!("{m_size}tb");
    }

    pub(crate) fn get_files_in_dir(path: &PathBuf, depth: u32) -> Result<Vec<PathBuf>, String> {
        if depth >= 128 {
            return Err(format!("File depth limit reached"));
        }

        if !path.is_dir() {
            return Err(format!("Can only get files from a directory"));
        }

        let mut files: Vec<PathBuf> = vec![];

        for entry in path
            .read_dir()
            .map_err(|e| format!("Failed to read directory '{}': {}", path.display(), e))?
        {
            if let Ok(entry) = entry {
                // If entry is directory, recursively get files in entry
                if entry.path().is_dir() {
                    files.append(&mut {
                        match FileUtils::get_files_in_dir(&entry.path(), depth + 1) {
                            Ok(t) => t,
                            Err(e) => {
                                return Err(format!(
                                    "Failed to get files in directory '{}': {}",
                                    entry.path().display(),
                                    e
                                ));
                            }
                        }
                    });
                } else {
                    files.push(entry.path());
                }
            }
        }

        return Ok(files);
    }

    pub(crate) fn path_to_relative(path: &PathBuf, pwd: &str) -> String {
        format!(
            "./{}",
            path.to_str()
                .expect("")
                .trim_start_matches(pwd)
                .trim_start_matches("\\")
                .replace("\\", "/")
        )
    }

    fn resolve_file_path(&self, dir_type: ProjectDirType, file_name: &str) -> PathBuf {
        let dir = match dir_type {
            ProjectDirType::Data => self.project_dirs.data_dir(),
            ProjectDirType::Config => self.project_dirs.config_dir(),
        };

        std::fs::create_dir_all(&dir).unwrap();
        dir.join(file_name)
    }

    fn create_file_if_not_exist<T: Serialize + Default>(
        path: &PathBuf,
    ) -> Result<bool, serde_json::Error> {
        if path.exists() {
            return Ok(false);
        }

        println!("Creating file: {}", path.display());
        let default: T = Default::default();
        std::fs::write(&path, serde_json::to_string(&default)?.trim_matches('"')).unwrap();

        Ok(true)
    }
}
