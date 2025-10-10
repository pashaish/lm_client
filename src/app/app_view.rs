use framework::Context;
use iced::{
    Background, Length, Theme,
    widget::{self, Container, Row, container},
};

use crate::widgets::{
    icon::{IconName, IconType},
    icon_button::IconButton,
};

use super::{App, app_state};

impl App {
    pub fn view(&self) -> iced::Element<'_, super::Message> {
        let mut main_row = Row::new().spacing(4).align_y(iced::Alignment::Start);

        main_row = main_row.push(self.selection_panel());
        main_row = main_row.push(
            iced::widget::Container::new(self.get_current_view(&self.context))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
        );

        Container::new(main_row).into()
    }

    fn selection_panel(&self) -> iced::Element<'_, super::Message> {
        let mut selection_panel = iced::widget::Column::new()
            .spacing(0)
            .height(Length::Fill)
            .align_x(iced::Alignment::Center);

        selection_panel = selection_panel.push(self.selection_panel_button(
            IconType::Regular(IconName::Message),
            app_state::View::Conversations,
        ));

        selection_panel = selection_panel.push(
            self.selection_panel_button(IconType::Solid(IconName::Box), app_state::View::Presets),
        );

        selection_panel = selection_panel.push(
            self.selection_panel_button(IconType::Solid(IconName::Gear), app_state::View::Settings),
        );

        Container::new(selection_panel)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();

                container::Style {
                    background: Some(Background::Color(
                        palette.primary.strong.color.scale_alpha(1.0),
                    )),
                    ..Default::default()
                }
            })
            .into()
    }

    fn selection_panel_button<'a>(
        &self,
        icon: IconType,
        view: app_state::View,
    ) -> iced::Element<'a, super::Message> {
        let is_selected = if self.current_view == view { 0.5 } else { 0.0 };

        IconButton::new(icon, super::Message::StartChangeView(view))
            .padding(8.0)
            .size(20.0)
            .style(move |theme: &Theme, status| {
                let palette = theme.extended_palette();

                widget::button::Style {
                    background: match status {
                        widget::button::Status::Pressed => Some(Background::Color(
                            palette.primary.base.color.scale_alpha(0.5 + is_selected),
                        )),
                        widget::button::Status::Hovered => Some(Background::Color(
                            palette.primary.base.color.scale_alpha(0.3 + is_selected),
                        )),
                        widget::button::Status::Active |
                        widget::button::Status::Disabled => Some(Background::Color(
                            palette.primary.base.color.scale_alpha(0.0 + is_selected),
                        )),
                    },
                    ..Default::default()
                }
            })
            .into()
    }

    fn get_current_view<'a>(&'a self, ctx: &'a Context) -> iced::Element<'a, super::Message> {
        match self.current_view {
            app_state::View::Conversations => self
                .conversations
                .view(ctx)
                .map(super::Message::Conversations),
            app_state::View::Presets => self.presets.view(ctx).map(super::Message::Presets),
            app_state::View::Settings => self.settings.view(ctx).map(super::Message::Settings),
        }
    }
}
