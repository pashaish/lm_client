use framework::{types::dto::ProviderDTO, Context};
use iced::{
    widget::{
        button, container, horizontal_space, pane_grid, text, vertical_space, Column, Container, Row, Scrollable, Text
    }, Element, Theme
};

use crate::{
    theme::styles,
    widgets::{button::Button, input::Input},
};

use super::{ProvidersSettings, providers_settings_state::Pane};

impl ProvidersSettings {
    pub fn view(&self, _ctx: &Context) -> Element<super::Message> {
        pane_grid::PaneGrid::new(&self.panes, |_, pane, _| match pane {
            Pane::ProvidersList => Container::new(self.providers_list())
                .style(|theme: &Theme| container::Style {
                    shadow: styles::fake_oneside_border(theme, &styles::Side::Right),
                    ..Default::default()
                })
                .into(),

            Pane::ProviderDetails => {
                let selected_provider = self
                    .selected_provider
                    .and_then(|id| self.providers.get(&id));

                if selected_provider.is_none() {
                    return Container::new(Text::new("No provider selected"))
                        .padding(10)
                        .align_x(iced::Alignment::Center)
                        .align_y(iced::Alignment::Center)
                        .into();
                }

                Container::new(self.provider_details())
                    .align_x(iced::Alignment::Center)
                    .style(|theme: &Theme| container::Style {
                        shadow: styles::fake_oneside_border(theme, &styles::Side::Left),
                        ..Default::default()
                    })
                    .into()
            }
        })
        .on_resize(5, super::Message::ResizePane)
        .into()
    }

    fn providers_list(&self) -> Element<super::Message> {
        let providers = self.get_sorted_providers();

        let mut list = iced::widget::Column::new().padding(10).spacing(10);

        for provider in providers {
            list = list.push(
                Button::new(Text::new(provider.name.clone()))
                    .on_press(super::Message::SelectProvider(Some(provider.id)))
                    .view()
                    .style(iced::widget::button::secondary)
                    .width(iced::Length::Fill),
            );
        }
        list = list.push(
            Container::new(self.view_add_provider())
            .padding(5),
        );

        Scrollable::new(list).style(styles::scrollable_style).into()
    }

    fn view_add_provider(&self) -> Element<super::Message> {
        let mut btn = Button::new(
            Text::new("Add Provider")
                .width(iced::Length::Fill)
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center),
        );

        let default_provider = ProviderDTO::default();
        if !self.providers.iter().any(|(_, provider)| provider.is_similar(&default_provider)) {
            btn = btn.on_press(super::Message::AddProvider);
        }

        btn
            .view()
            .width(iced::Length::Fill)
            .into()
    }

    fn provider_details(&self) -> Element<'_, super::Message> {
        Container::new(
            Column::new()
                .padding(10)
                .spacing(10)
                .push(
                    Input::new(&self.temp_provider.name)
                        .on_change(super::Message::UpdateProviderName)
                        .label("Provider Name"),
                )
                .push(
                    Input::new(&self.temp_provider.url)
                        .on_change(super::Message::UpdateProviderUrl)
                        .label("Provider URL"),
                )
                .push(
                    Input::new(&self.temp_provider.api_key)
                        .on_change(super::Message::UpdateProviderApiKey)
                        .label("API Key")
                        .secure(),
                )
                .push(
                    Input::new(&self.temp_provider.default_model)
                        .on_change(super::Message::UpdateProviderDefaultModel)
                        .label("Default Model"),
                )
                .push(vertical_space())
                .push(
                    Row::new()
                        .spacing(10)
                        .push(
                            Button::new(
                                Text::new("Save")
                                    .align_x(iced::Alignment::Center)
                                    .align_y(iced::Alignment::Center),
                            )
                            .on_press(super::Message::SaveProvider),
                        )
                        .push(horizontal_space())
                        .push(if self.is_unsaved_changes() {
                            Text::new("Unsaved changes").style(text::danger).size(20)
                        } else {
                            Text::new("").size(20)
                        })
                        .push(
                            Button::new(
                                Text::new("Delete")
                                    .align_x(iced::Alignment::Center)
                                    .align_y(iced::Alignment::Center),
                            )
                            .on_press(super::Message::DeleteProvider)
                            .view()
                            .style(button::danger),
                        ),
                ),
        )
        .into()
    }
}
