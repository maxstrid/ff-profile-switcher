use iced::theme::{self, Palette};
use iced::widget::{button, column, container, text};
use iced::{
    executor, keyboard, subscription, Alignment, Application, Command, Element, Event, Length,
    Renderer, Theme,
};

mod profile;
mod style;

use style::{rgb, ButtonStyle, LeftContainerStyle};

#[derive(Debug, Clone)]
pub struct ProfileSwitcher {
    // TODO: Make this a hashmap containing the profile as the key and the
    // image as the value.
    profiles: Vec<String>,
    should_exit: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ProfileOpened,
    ProfilePressed(String),
    ShouldExit,
}

impl ProfileSwitcher {
    pub fn settings() -> iced::Settings<()> {
        iced::Settings {
            window: iced::window::Settings {
                size: (800, 600),
                position: iced::window::Position::Centered,
                resizable: false,
                decorations: false,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Application for ProfileSwitcher {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let profiles = profile::get_profiles();
        (
            ProfileSwitcher {
                profiles,
                should_exit: false,
            },
            Command::none(),
        )
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
            Message::ShouldExit => self.should_exit = true,
        };

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let rows = column::<Message, Renderer>(
            self.profiles
                .iter()
                .map(|profile| {
                    button(
                        text(profile.as_str())
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    )
                    .on_press(Message::ProfilePressed(profile.clone()))
                    .style(theme::Button::Custom(Box::new(ButtonStyle::default())))
                    .into()
                })
                .collect(),
        )
        .spacing(10)
        .padding(10)
        .align_items(Alignment::Fill);

        container(rows)
            .height(Length::Fill)
            .center_y()
            .style(theme::Container::Custom(Box::new(
                LeftContainerStyle::default(),
            )))
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::events_with(|event, _status| match event {
            Event::Keyboard(key_event) => {
                if let keyboard::Event::KeyPressed {
                    key_code,
                    modifiers: _,
                } = key_event
                {
                    if key_code == keyboard::KeyCode::Escape {
                        return Some(Message::ShouldExit);
                    } else {
                        return None;
                    }
                }
                None
            }
            _ => None,
        })
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
