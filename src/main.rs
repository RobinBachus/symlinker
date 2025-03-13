mod file_utils;

use directories::{ProjectDirs};

fn main() {
    let project_dirs = ProjectDirs::from(
        "xyz",
        "server1rb",
        "symlinker"
    ).unwrap();
    println!("ProjectDirs: {:?}", project_dirs);
}
