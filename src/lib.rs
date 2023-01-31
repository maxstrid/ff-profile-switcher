use iced::theme::{self, Palette};
use iced::widget::{button, column, container, text};
use iced::window;
use iced::{
    executor, keyboard, subscription, Alignment, Application, Command, Element, Event, Length,
    Renderer, Theme,
};

mod profile;
mod style;

use style::*;

#[derive(Debug, Clone)]
pub enum Message {
    ProfilePressed(String),
    ShouldExit,
    SwitchTheme,
}

#[derive(Debug)]
pub struct ProfileManager {
    profiles: Vec<String>,
    current_theme: AppTheme,
    light_theme: AppTheme,
    dark_theme: AppTheme,
}

impl Application for ProfileManager {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ProfileManager, Command<Message>) {
        let profiles = profile::get_profiles();

        let light_theme = AppTheme {
            main_col: rgb(228, 240, 247),
            accent_col: rgb(200, 202, 204),
            focus_col: rgb(255, 150, 51),
            text_col: rgb(0, 0, 0),
        };

        let dark_theme = AppTheme {
            main_col: rgb(33, 33, 33),
            accent_col: rgb(48, 47, 54),
            focus_col: rgb(255, 150, 51),
            text_col: rgb(255, 255, 255),
        };

        (
            ProfileManager {
                profiles,
                current_theme: dark_theme,
                light_theme,
                dark_theme,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Firefox Profile Manager")
    }

    fn theme(&self) -> Theme {
        self.current_theme.theme()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ProfilePressed(profile) => {
                Command::perform(profile::open_profile(profile), |_| Message::ShouldExit)
            }
            Message::ShouldExit => window::close(),
            Message::SwitchTheme => {
                if self.current_theme == self.dark_theme {
                    self.current_theme = self.light_theme;
                } else {
                    self.current_theme = self.dark_theme;
                };

                Command::none()
            }
        }
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
                    .style(theme::Button::Custom(Box::new(self.current_theme)))
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
            .center_x()
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
}

impl ProfileManager {
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
