use iced::widget::button;
use iced::widget::container;
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

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let bg_color = rgb(50, 50, 50);

        button::Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: self.text_color,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        let bg_color = rgb(255, 160, 25);

        button::Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: self.text_color.inverse(),
            border_radius: 5.0,
            border_width: 2.0,
            border_color: rgb(255, 255, 255),
            ..Default::default()
        }
    }
}

pub struct LeftContainerStyle {
    bg_color: Color,
    text_color: Color,
}

impl Default for LeftContainerStyle {
    fn default() -> Self {
        Self {
            bg_color: rgb(60, 60, 60),
            text_color: rgb(255, 255, 255),
        }
    }
}

impl container::StyleSheet for LeftContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.bg_color)),
            text_color: Some(self.text_color),
            ..Default::default()
        }
    }
}

// Helper function to convert normal rgb to a color
pub fn rgb(r: i32, g: i32, b: i32) -> Color {
    Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
