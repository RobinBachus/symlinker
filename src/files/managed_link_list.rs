use crate::enums::project_dir_type::ProjectDirType;
use crate::files::managed_link::ManagedLink;
use crate::utils::file_utils::FileUtils;
use serde::{Deserialize, Serialize};

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
        file_utils
            .parse_file(ProjectDirType::Data, "managed_links.json")
            .unwrap()
    }

    pub fn save(&self, file_utils: &FileUtils) {
        file_utils
            .save_file(ProjectDirType::Data, "managed_links.json", self)
            .unwrap();
    }

    pub fn get_last_id(&self) -> u32 {
        self.managed_links
            .iter()
            .map(|link| link.id)
            .max()
            .unwrap_or(0)
    }
}
