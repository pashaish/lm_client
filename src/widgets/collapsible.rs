use iced::{
    Border, Element, Length, Theme, alignment,
    widget::{button, column, container, row, text},
};

struct Collapsible<'a, Message: Clone + 'a> {
    title: String,
    content: Element<'a, Message>,
    expanded: bool,
    on_toggle: Message,
}

impl<'a, Message: Clone + 'a> Collapsible<'a, Message> {
    pub fn new(
        title: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
        default_expanded: bool,
        default_on_toggle: Message,
    ) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            expanded: default_expanded,
            on_toggle: default_on_toggle,
        }
    }

    pub fn view(self) -> Element<'a, Message> {
        let toggle_icon = if self.expanded { "-" } else { "+" };

        let mut btn =
            button(text(toggle_icon))
                .height(28)
                .width(28)
                .style(|theme: &Theme, status| button::Style {
                    text_color: iced::widget::button::text(theme, status).text_color,
                    ..Default::default()
                });

        btn = btn.on_press(self.on_toggle);

        let header = row![btn, text(self.title).size(16),]
            .spacing(5)
            .align_y(alignment::Alignment::Center);

        let mut content = column![header].padding(8);

        if self.expanded {
            content = content.push(self.content);
        }

        let container = container(content)
            .width(Length::Fill)
            .style(|theme: &Theme| container::Style {
                border: Border {
                    color: theme.palette().text,
                    width: 1.0,
                    radius: 3.0.into(),
                },
                ..Default::default()
            })
            .padding(0);

        container.into()
    }
}

pub fn collapsible<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    content: impl Into<Element<'a, Message>>,
    expanded: bool,
    on_toggle: Message,
) -> Element<'a, Message> {
    Collapsible::new(title, content, expanded, on_toggle).view()
}
