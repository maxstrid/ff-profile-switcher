use std::fs;
use std::process::{Command, Stdio};

use directories::UserDirs;

pub fn open_profile(profile_name: String) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(format!("firefox -P {profile_name}"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Coudln't run firefox")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("firefox -P {profile_name}"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Couldn't run firefox")
    };
}

pub fn get_profiles() -> Vec<String> {
    let user_dirs = UserDirs::new().expect("Couldn't get directory for user");
    let home_dir = user_dirs
        .home_dir()
        .to_str()
        .expect("Couldn't get home directory for user");

    let target: String;

    if cfg!(target_os = "windows") {
        target = format!(r#"{home_dir}\AppData\Roaming\Mozilla\Firefox\Profiles"#)
    } else {
        target = format!("{home_dir}/.mozilla/firefox");
    }

    fs::read_dir(target)
        .expect("Couldn't read directory")
        .filter(|entry| {
            let file = match entry {
                Ok(file) => file,
                Err(_) => return false,
            };

            let metadata = match file.metadata() {
                Ok(meta) => meta,
                Err(_) => return false,
            };

            if !metadata.is_dir() || file.path().extension().is_none() {
                return false;
            }

            true
        })
        .map(|entry| {
            let file = match entry {
                Ok(file) => file,
                Err(_) => return String::from(""),
            };

            match file.path().extension() {
                Some(data) => {
                    let data = match data.to_str() {
                        Some(data) => data,
                        None => return String::from("")
                    };
                    String::from(data.to_string())
                },
                None => return String::from("")
            }
        })
        .collect::<Vec<String>>()
}
