use iced::{Application, Settings};
use ff_profile_switcher::ProfileSwitcher;

fn main() -> iced::Result {
    ProfileSwitcher::run(Settings::default())
}
