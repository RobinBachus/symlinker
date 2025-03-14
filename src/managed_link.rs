use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ManagedLink {
    pub id: u32,
    pub original_path: String,
    pub symlink_path: String,
    pub creation_date: String,
    pub last_modified: String,
    pub link_type: String,
}

impl Display for ManagedLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {}\nOriginal path: {}\nSymlink path: {}\nCreation date: {}\nLast modified: {}\nLink type: {}",
            self.id,
            self.original_path,
            self.symlink_path,
            self.creation_date,
            self.last_modified,
            self.link_type
        )
    }
}

