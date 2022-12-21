use iced::theme::{self, Palette};
use iced::widget::{button, column, container, row};
use iced::{executor, Alignment, Application, Command, Element, Length, Renderer, Theme};

mod profile;
mod style;

use style::{rgb, ButtonStyle};

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
        Theme::custom(Palette {
            background: rgb(26, 26, 25),
            text: rgb(255, 255, 255),
            primary: rgb(39, 50, 78),
            success: rgb(50, 130, 40),
            danger: rgb(130, 26, 15),
        })
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
        let rows = row::<Message, Renderer>(
            self.profiles
                .iter()
                .map(|profile| {
                    button(profile.as_str())
                        .on_press(Message::ProfilePressed(profile.clone()))
                        .style(theme::Button::Custom(Box::new(ButtonStyle::default())))
                        .into()
                })
                .collect(),
        )
        .spacing(10)
        .padding(10)
        .align_items(Alignment::Center);

        container(rows)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
