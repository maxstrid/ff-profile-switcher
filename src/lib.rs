use std::process::{Command, Stdio};

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
    if cfg!(target_os = "windows") {
        todo!()
    } else {
        todo!()
    }
}
