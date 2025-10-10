use iced::{
    Element, Theme,
    widget::{Button, button},
};

use super::icon::{Icon, IconType};

type IconButtonStyle =
    Option<Box<dyn Fn(&Theme, button::Status) -> iced::widget::button::Style + 'static>>;

pub struct IconButton<T> {
    icon: IconType,
    message: T,
    alpha: f32,
    padding: f32,
    size: f32,
    style: IconButtonStyle,
    disabled: bool,
}

impl<T> IconButton<T> {
    pub fn new(icon: IconType, message: T) -> Self {
        Self {
            icon,
            message,
            alpha: 1.0,
            size: 16.0,
            padding: 5.0,
            style: None,
            disabled: false,
        }
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub const fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub const fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn style<F>(mut self, style: F) -> Self
    where
        F: Fn(&Theme, button::Status) -> iced::widget::button::Style + 'static,
    {
        self.style = Some(Box::new(style));
        self
    }

    pub fn view(self) -> Button<'static, T> {
        let mut btn = button(
            Icon::new(self.icon)
                .alpha(self.alpha)
                .view()
                .size(self.size),
        )
        .padding(self.padding);

        if !self.disabled {
            btn = btn.on_press(self.message);
        }

        if let Some(stl) = self.style {
            btn = btn.style(move |theme, status| stl(theme, status));
        } else {
            btn = btn.style(default_style);
        };

        btn
    }
}

fn default_style(theme: &Theme, status: button::Status) -> iced::widget::button::Style {
    match status {
        button::Status::Hovered |
        button::Status::Pressed => hovered(theme),
        _ => iced::widget::button::text(theme, status),
    }
}

fn hovered(theme: &Theme) -> iced::widget::button::Style {
    let palette = theme.extended_palette();

    iced::widget::button::Style {
        background: Some(palette.primary.base.color.into()),
        ..Default::default()
    }
}

impl<T> From<IconButton<T>> for Element<'static, T>
where
    T: Clone + 'static,
{
    fn from(icon_button: IconButton<T>) -> Self {
        icon_button.view().into()
    }
}
