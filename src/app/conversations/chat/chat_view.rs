use framework::Context;
use iced::{
    Element, Theme,
    widget::{Button, Container, MouseArea, Row, container, text},
};

// use crate::{theme::styles, widgets::{icon::{IconName, IconType}, icon_button::IconButton}};

use crate::theme::styles;

use super::Chat;

impl Chat {
    pub fn view<'a>(&'a self, settings_expanded: bool, ctx: &'a Context) -> Element<'a, super::Message> {
        let mut main_column = iced::widget::Column::new().spacing(10);

        main_column = main_column
            .push(self.chat_title(settings_expanded))
            .push(self.view_messages(ctx))
            .push(self.view_texteditor(ctx));

        Container::new(main_column)
            .style(|theme: &Theme| container::Style {
                shadow: styles::fake_oneside_border(theme, &styles::Side::Left),
                ..Default::default()
            })
            .into()
    }

    fn chat_title(&self, settings_expanded: bool) -> Element<super::Message> {
        let chat_name = self.chat.as_ref().map_or_else(|| "Loading...".to_string(), |chat| chat.name.clone());

        let icon_btn = if settings_expanded {
                // ?TODO: NEED UPDATE

                MouseArea::new(
                    text(">")
                        .width(iced::Length::Fill)
                        .align_x(iced::alignment::Horizontal::Center)
                        .align_y(iced::alignment::Vertical::Center),
                )
                .on_press(super::Message::ToggleSettings(false))
        } else {
                // ?TODO: NEED UPDATE
                MouseArea::new(
                    text("<")
                        .width(iced::Length::Fill)
                        .align_x(iced::alignment::Horizontal::Center)
                        .align_y(iced::alignment::Vertical::Center),
                )
                .on_press(super::Message::ToggleSettings(true))
        };

        Row::new()
            .align_y(iced::alignment::Vertical::Center)
            .padding(10)
            .spacing(10)
            .push(
                iced::widget::Text::new(chat_name)
                    .width(iced::Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .push(icon_btn)
            .into()
    }
}
