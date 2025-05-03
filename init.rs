use std::fs;
use std::path::Path;

/// Initializes a new GVC repository in the current directory.
pub fn init_repository() {
    let gvc_dir = Path::new(".gvc");
    let objects = gvc_dir.join("objects");
    let refs = gvc_dir.join("refs");

    fs::create_dir_all(&objects).expect("Failed to create objects/");
    fs::create_dir_all(&refs).expect("Failed to create refs/");
    fs::write(gvc_dir.join("HEAD"), "ref: refs/main").expect("Failed to write HEAD");

    println!("Initialized an empty GVC repository");
}
