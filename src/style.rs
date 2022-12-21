use iced::widget::button::{Appearance, StyleSheet};
use iced::{Background, Color, Theme};

pub struct ButtonStyle {
    pub text_color: Color,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            text_color: rgb(255, 255, 255),
        }
    }
}

impl StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        let bg_color = rgb(50, 50, 50);

        Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: self.text_color,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        let bg_color = rgb(255, 160, 25);

        Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: self.text_color,
            ..Default::default()
        }
    }
}

// Helper function to convert normal rgb to a color
pub fn rgb(r: i32, g: i32, b: i32) -> Color {
    Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
