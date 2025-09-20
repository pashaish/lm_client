use iced::{Theme, widget::button::Style};

pub struct Button<'a, T>
where
    T: Clone + 'a,
{
    content: iced::Element<'a, T>,
    on_press: Option<T>,
}

impl<'a, T> Button<'a, T>
where
    T: Clone + 'a,
{
    pub fn new(content: impl Into<iced::Element<'a, T>>) -> Self {
        Button {
            content: content.into(),
            on_press: None,
        }
    }

    pub fn on_press(mut self, f: T) -> Self {
        self.on_press = Some(f);
        self
    }

    pub fn view(self) -> iced::widget::Button<'a, T> {
        let mut btn = iced::widget::Button::new(self.content);

        if let Some(on_press) = self.on_press {
            btn = btn.on_press(on_press);
        }

        btn = btn.style(|theme: &Theme, status| Style {
            background: iced::widget::button::primary(theme, status).background,
            text_color: iced::widget::button::secondary(theme, status).text_color,
            ..Default::default()
        });

        btn
    }
}

impl<'a, T> From<Button<'a, T>> for iced::Element<'a, T>
where
    T: Clone + 'a,
{
    fn from(button: Button<'a, T>) -> Self {
        button.view().into()
    }
}
