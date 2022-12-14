use std::fs;
use std::process::{Command, Stdio};

use directories::UserDirs;

mod error;
use error::Error;

#[cfg(target_family = "windows")]
const PROFILE_DIR: &str = r#"\AppData\Roaming\Mozilla\Firefox\Profiles"#;

#[cfg(target_family = "unix")]
const PROFILE_DIR: &str = "/.mozilla/firefox";

pub async fn open_profile(profile_name: String) {
    async {
        let mut env: &str = "sh";
        let mut arg: &str = "-c";

        if cfg!(target_os = "windows") {
            env = "cmd";
            arg = "/C";
        }

        Command::new(env)
            .arg(arg)
            .arg(format!("firefox -P {profile_name}"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Couldn't run firefox");
    }
    .await
}

pub fn get_profiles() -> Vec<String> {
    let user_dirs = UserDirs::new().expect("Couldn't get directory for user");
    let home_dir = user_dirs
        .home_dir()
        .to_str()
        .expect("Couldn't get home directory for user");

    let target = format!("{home_dir}{PROFILE_DIR}");

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
                        None => return String::from(""),
                    };
                    String::from(data.to_string())
                }
                None => return String::from(""),
            }
        })
        .collect::<Vec<String>>()
}
