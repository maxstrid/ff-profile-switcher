use directories::ProjectDirs;

fn main() {
    let ffdir = ProjectDirs::from("io.github.maxstrid", "FFPViewier", "FF Profile Viewier").unwrap();

    let dir = ffdir.data_dir().to_str().unwrap();

    std::fs::create_dir(dir).unwrap();

    println!("cargo:rerun-if-changed={dir}");
}