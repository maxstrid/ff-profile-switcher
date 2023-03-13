use iced::alignment;
use iced::theme;
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
    SelectNext,
    OpenSelected,
}

#[derive(Debug)]
pub struct ProfileManager {
    profiles: Vec<String>,
    selected_profile: Option<usize>,
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
            accent_col: rgb(48, 48, 48),
            focus_col: rgb(255, 150, 51),
            text_col: rgb(255, 255, 255),
        };

        (
            ProfileManager {
                profiles,
                selected_profile: None,
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
            Message::SelectNext => {
                if let Some(index) = self.selected_profile {
                    if index < self.profiles.len() && index + 1 != self.profiles.len() {
                        self.selected_profile = Some(index + 1);
                    } else {
                        self.selected_profile = Some(0);
                    }
                } else {
                    self.selected_profile = Some(0);
                }

                Command::none()
            }
            Message::OpenSelected => {
                if let Some(index) = self.selected_profile {
                    if let Some(profile_name) = self.profiles.get(index) {
                        return Command::perform(
                            profile::open_profile(profile_name.to_string()),
                            |_| Message::ShouldExit,
                        );
                    }
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let get_theme = |profile: String, selected_profile: Option<usize>| -> theme::Button {
            if let Some(profile_index) = selected_profile {
                if profile == *self.profiles.get(profile_index).unwrap() {
                    return theme::Button::Custom(Box::new(self.current_theme.selected()));
                }
            }

            theme::Button::Custom(Box::new(self.current_theme))
        };

        let profile_buttons: Vec<Element<Message, Renderer>> = self
            .profiles
            .iter()
            .map(|profile| {
                button(text(profile.as_str()).horizontal_alignment(alignment::Horizontal::Center))
                    .on_press(Message::ProfilePressed(profile.clone()))
                    .style(get_theme(profile.to_string(), self.selected_profile))
                    .into()
            })
            .collect();

        let columns = column::<Message, Renderer>(profile_buttons)
            .spacing(10)
            .align_items(Alignment::Fill);

        let profile_container = container(columns)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y();

        let main_column = column::<Message, Renderer>(vec![
            profile_container.into(),
            button("Switch Theme")
                .style(theme::Button::Custom(Box::new(self.current_theme)))
                .on_press(Message::SwitchTheme)
                .into(),
        ])
        .spacing(10)
        .padding(10);

        container(main_column)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
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
                    match key_code {
                        keyboard::KeyCode::Escape => return Some(Message::ShouldExit),
                        keyboard::KeyCode::Q => return Some(Message::ShouldExit),
                        keyboard::KeyCode::Tab => return Some(Message::SelectNext),
                        keyboard::KeyCode::Enter => return Some(Message::OpenSelected),
                        _ => return None,
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
