use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct ManagedLink {
    pub id: u32,
    pub original_path: String,
    pub symlink_path: String,
    pub creation_date: String,
    pub last_modified: String,
}

impl Display for ManagedLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {}\nOriginal path: {}\nSymlink path: {}\nCreation date: {}\nLast modified: {}",
            self.id, self.original_path, self.symlink_path, self.creation_date, self.last_modified,
        )
    }
}
