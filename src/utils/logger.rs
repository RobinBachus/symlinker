use crate::enums::project_dir_type::ProjectDirType;

use super::file_utils::FileUtils;

const FILE_NAME: &str = "lastRun.log";

pub(crate) struct Logger;

impl Logger {
    pub(crate) fn clear_or_create_file(file_utils: &FileUtils) -> Result<(), std::io::Error> {
        file_utils.write(ProjectDirType::Data, FILE_NAME, &"")?;
        Ok(())
    }

    pub(crate) fn log_to_file(file_utils: &FileUtils, message: &str) -> Result<(), std::io::Error> {
        file_utils.append(ProjectDirType::Data, FILE_NAME, &format!("{message}\n"))
    }
}
