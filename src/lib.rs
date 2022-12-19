use iced::{executor, Application, Theme, Command, Element};
use iced::widget::{button, column};

mod profile;

#[derive(Debug, Clone)]
pub struct ProfileSwitcher {
    profiles: Vec<String>
}

#[derive(Debug, Clone)]
pub enum Message {
    ProfileOpened,
    ProfilePressed(String),
}

impl Application for ProfileSwitcher {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let profiles = profile::get_profiles();
        (ProfileSwitcher { profiles }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Firefox Profile Switcher")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ProfilePressed(profile) => return Command::perform(profile::open_profile(profile), |_| Message::ProfileOpened),
            Message::ProfileOpened => ()
        };

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column(self.profiles
                .iter()
                .map(|profile| {
                    button(profile.as_str()).on_press(Message::ProfilePressed(profile.clone())).into()
                }).collect()
        )
        .align_items(iced::Alignment::Center)
        .into()
    }
}
