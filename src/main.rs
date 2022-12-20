use ff_profile_switcher::ProfileSwitcher;
use iced::{Application, Settings};

fn main() -> iced::Result {
    ProfileSwitcher::run(Settings::default())
}
