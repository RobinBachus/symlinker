use crate::enums::project_dir_type::ProjectDirType;

use super::file_utils::FileUtils;

const FILE_NAME: &str = "lastRun.log";

pub(crate) struct Logger;

impl Logger {
    /// Creates the log file or clears it if already created.
    /// # Arguments:
    /// * `file_utils`: A `FileUtils` instance to use for IO operations. 
    /// # Returns:
    /// `Result<(), std::io::error>` 
    pub(crate) fn clear_or_create_file(file_utils: &FileUtils) -> Result<(), std::io::Error> {
        file_utils.write(ProjectDirType::Data, FILE_NAME, &"")?;
        Ok(())
    }

    /// Appends a message at the end of the log file.
    /// # Arguments:
    /// * `file_utils`: A `FileUtils` instance to use for IO operations. 
    /// * `message`: The message to append to the logs.
    /// # Returns:
    /// `Result<(), std::io::error>` 
    pub(crate) fn log_to_file(file_utils: &FileUtils, message: &str) -> Result<(), std::io::Error> {
        file_utils.append(ProjectDirType::Data, FILE_NAME, &format!("{message}\n"))
    }
}
