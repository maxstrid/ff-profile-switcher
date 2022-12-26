use ff_profile_switcher::ProfileSwitcher;
use iced::Application;

fn main() -> iced::Result {
    ProfileSwitcher::run(ProfileSwitcher::settings())
}
