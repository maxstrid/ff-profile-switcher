use iced::widget::button::{Appearance, StyleSheet};
use iced::{Background, Color, Theme};

pub struct ButtonStyle {
    pub text_color: Color,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
        }
    }
}

impl StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        let bg_color = Color::from_rgb(0.2, 0.2, 0.2);

        Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: self.text_color,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        let bg_color = Color::from_rgb(1.0, 0.6, 0.1);

        Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: self.text_color,
            ..Default::default()
        }
    }
}
