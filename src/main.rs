use ff_profile_switcher::ProfileManager;
use iced::Application;

fn main() -> iced::Result {
    ProfileManager::run(ProfileManager::settings())
}
