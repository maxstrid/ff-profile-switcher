use iced::widget::{button, column, container};
use iced::{executor, Alignment, Application, Command, Element, Length, Renderer, Theme};

mod profile;

#[derive(Debug, Clone)]
pub struct ProfileSwitcher {
    // TODO: Make this a hashmap containing the profile as the key and the
    // image as the value.
    profiles: Vec<String>,
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
            Message::ProfilePressed(profile) => {
                return Command::perform(profile::open_profile(profile), |_| Message::ProfileOpened)
            }
            Message::ProfileOpened => (),
        };

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        container(
            column::<Message, Renderer>(
                self.profiles
                    .iter()
                    .map(|profile| {
                        button(profile.as_str())
                            .on_press(Message::ProfilePressed(profile.clone()))
                            .into()
                    })
                    .collect(),
            )
            .spacing(10)
            .padding(10)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
