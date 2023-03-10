use iced::theme::Palette;
use iced::widget::button;
use iced::{Background, Color, Theme};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AppTheme {
    pub main_col: Color,
    pub accent_col: Color,
    pub focus_col: Color,
    pub text_col: Color,
}

impl AppTheme {
    pub fn theme(&self) -> Theme {
        Theme::custom(Palette {
            background: self.main_col,
            text: self.text_col,
            primary: self.accent_col,
            success: rgb(0, 0, 255),
            danger: rgb(255, 0, 0),
        })
    }

    pub fn selected(&self) -> SelectedTheme {
        SelectedTheme::new(*self)
    }
}

impl button::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.accent_col)),
            text_color: self.text_col,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.focus_col)),
            text_color: self.text_col.inverse(),
            border_radius: 5.0,
            border_width: 2.0,
            border_color: rgb(255, 255, 255),
            ..Default::default()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SelectedTheme {
    pub main_col: Color,
    pub accent_col: Color,
    pub focus_col: Color,
    pub text_col: Color,
}

impl SelectedTheme {
    pub fn new(theme: AppTheme) -> SelectedTheme {
        SelectedTheme {
            main_col: theme.main_col,
            accent_col: theme.accent_col,
            focus_col: theme.focus_col,
            text_col: theme.text_col,
        }
    }
}

impl button::StyleSheet for SelectedTheme {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.accent_col)),
            text_color: self.text_col,
            border_radius: 2.0,
            border_width: 1.0,
            border_color: self.focus_col,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.focus_col)),
            text_color: self.text_col.inverse(),
            border_radius: 5.0,
            border_width: 2.0,
            border_color: rgb(255, 255, 255),
            ..Default::default()
        }
    }
}

// Helper function to convert normal rgb to a color
pub fn rgb(r: i32, g: i32, b: i32) -> Color {
    Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
