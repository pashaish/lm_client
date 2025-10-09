
use crate::{
    theme::styles,
    // widgets::icon::{Icon, IconName, IconType},
};
use framework::Context;
use iced::{
    Element, Theme,
    widget::{
        Column, Container, Row, Scrollable, Text, container, space, text
        // horizontal_space,
    },
};

use super::Settings;

impl Settings {
    pub fn view(&self, ctx: &Context) -> Element<super::Message> {
        self.view_conversation(ctx)
    }

    fn view_conversation(&self, ctx: &Context) -> Element<super::Message> {
        Container::new(
            Scrollable::new(
                Column::new()
                    .spacing(10)
                    .width(iced::Length::Fill)
                    .align_x(iced::Alignment::Center)
                    .push(
                        Text::new(format!("Settings: {}", self.conversation.name))
                            .align_x(iced::Alignment::Center)
                            .align_y(iced::Alignment::Center),
                    )
                    .push(self.groups(vec![
                        ("Basic", self.basic.view(ctx).map(super::Message::Basic)),
                        ("RAG", self.rag.view(ctx).map(super::Message::Rag)),
                        (
                            "Summary",
                            self.summary.view(ctx).map(super::Message::Summary),
                        ),
                    ])),
            )
            .style(styles::scrollable_style),
        )
        .padding(16)
        .width(iced::Length::Fill)
        .style(|theme: &Theme| container::Style {
            shadow: styles::fake_oneside_border(theme, &styles::Side::Left),
            ..Default::default()
        })
        .into()
    }

    fn groups<'a>(
        &'a self,
        groups: Vec<(&'a str, Element<'a, super::Message>)>,
    ) -> Element<'a, super::Message> {
        let mut column = Column::new()
            .spacing(10)
            .width(iced::Length::Fill)
            .align_x(iced::Alignment::Start);

        for (label, element) in groups {
            let is_expanded = *self
                .groups_expaned
                .get(label)
                .unwrap_or(&false);

            column = column.push(
                Container::new(
                    iced::widget::Button::new(
                        Row::new()
                            .align_y(iced::alignment::Vertical::Center)
                            .padding(1)
                            .spacing(4)
                // +TODO: NEED UPDATE

                            .push(
                                text(if is_expanded { "📂" } else { "📁" }).size(12.0)
                            )

                            .push(Text::new(label).size(12).style(|theme: &Theme| {
                                let palette = theme.extended_palette();
                                iced::widget::text::Style {
                                    color: Some(palette.secondary.base.text.scale_alpha(0.3)),
                                }
                            })),
                    )
                    .width(iced::Length::Fill)
                    .style(iced::widget::button::text)
                    .on_press(super::Message::ToggleGroup(label.to_string())),
                )
                .width(iced::Length::Fill)
                .style(|theme: &Theme| container::Style {
                    shadow: styles::fake_oneside_border_primary(theme, &styles::Side::Bottom),
                    ..Default::default()
                }),
            );

            if is_expanded {
                column = column.push(Row::new().push(space::horizontal().width(10)).push(element));
            }
        }

        column.into()
    }
}
