use std::path::Path;
use std::fs;

use directories::ProjectDirs;

fn main() {
    let ffdir =
        ProjectDirs::from("io.github.maxstrid", "FFPViewier", "FF Profile Viewier").unwrap();

    let dir = ffdir.data_dir().to_str().unwrap();

    if !Path::new(dir).exists() {
        fs::create_dir(dir).unwrap();
    }

    let mut path = "/resources/";

    if cfg!(target_os = "windows") {
        path = r#"\Resources\"#
    }

    let resource_dir = format!("{dir}{path}");

    if !Path::new(&resource_dir).exists() {
        fs::create_dir(&resource_dir).unwrap();
    }

    if !Path::new(&format!("{resource_dir}placeholder.png")).exists() {
        fs::copy("resources/placeholder.png", format!("{resource_dir}placeholder.png")).unwrap();
    }

    println!("cargo:rerun-if-changed={dir}");
    println!("cargo:rerun-if-changed={resource_dir}");
}
