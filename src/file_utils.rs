use std::path::{Path, PathBuf};
use directories::{ProjectDirs};

struct FileUtils {
    data_dir: PathBuf,
}

impl FileUtils {
    fn new() -> FileUtils {
        let project_dirs = ProjectDirs::from(
            "xyz",
            "server1rb",
            "symlinker"
        ).unwrap();

        FileUtils {
            data_dir: project_dirs.data_dir().to_path_buf(),
        }
    }

    fn get_data_dir(&self) -> &Path {
        &self.data_dir
    }
}