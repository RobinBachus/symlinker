use serde::{Deserialize, Serialize};
use crate::utils::file_utils::{FileUtils, ProjectDirType};
use crate::files::managed_link::ManagedLink;

#[derive(Serialize, Deserialize)]
pub struct ManagedLinkList {
    pub managed_links: Vec<ManagedLink>,
}

impl Default for ManagedLinkList {
    fn default() -> Self {
        ManagedLinkList {
            managed_links: vec![],
        }
    }
}

impl ManagedLinkList {
    pub fn load(file_utils: &FileUtils) -> ManagedLinkList {
        file_utils.parse_file(ProjectDirType::Data, "managed_links.json").unwrap()
    }

    pub fn save(&self, file_utils: &FileUtils) {
        file_utils.save_file(ProjectDirType::Data, "managed_links.json", self);
    }
}