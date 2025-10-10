use framework::{Context, types::dto::PresetDTO};
use iced::{
    Element, Padding, Theme,
    widget::{
        Column, Container, Row, Scrollable, Slider, Text, TextEditor, TextInput, button, container,
        horizontal_space, pane_grid, text,
    },
};

use crate::{
    theme::styles::{self, scrollable_style},
    widgets::button::Button,
};

use super::Presets;

impl Presets {
    pub fn view(&self, _ctx: &Context) -> Element<super::Message> {
        let main_row = pane_grid::PaneGrid::new(&self.panes, |_, pane, _| match pane {
            super::presets_state::Pane::PresetsList => self.presets_list().into(),
            super::presets_state::Pane::PresetDetails => {
                Container::new(self.selected_preset.map_or_else(
                    || Text::new("No preset selected").into(),
                    |preset_id| {
                        let preset = self.presets.get(&preset_id).expect("Preset not found");

                        self.preset_details(&preset.clone())
                    },
                ))
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center)
                .style(|theme: &Theme| container::Style {
                    shadow: styles::fake_oneside_border(theme, &styles::Side::Left),
                    ..Default::default()
                })
                .into()
            }
        })
        .on_resize(5, super::Message::PaneResize);

        Container::new(main_row).into()
    }

    fn presets_list<'a>(&'a self) -> Element<'a, super::Message> {
        let mut column = iced::widget::Column::new()
            .spacing(10)
            .width(iced::Length::Fill);

        for preset in self.get_sorted_presets() {
            let name = preset.name.clone();

            let msg = super::Message::SelectPreset(Some(preset.id));

            let mut btn = Button::new(Text::new(name));
            if self.selected_preset != Some(preset.id) {
                btn = btn.on_press(msg);
            }

            let btn: Element<'a, super::Message> = btn
                .view()
                .style(iced::widget::button::secondary)
                .width(iced::Length::Fill)
                .into();

            column = column.push(btn);
        }

        column = column.push(self.view_add_preset());

        Container::new(
            Column::new()
                .push(
                    Scrollable::new(Container::new(column).padding(5))
                        .style(scrollable_style)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
                .height(iced::Length::Fill)
                .width(iced::Length::FillPortion(1)),
        )
        .style(|theme: &Theme| container::Style {
            shadow: styles::fake_oneside_border(theme, &styles::Side::Right),
            ..Default::default()
        })
        .into()
    }

    fn preset_details(&self, preset: &PresetDTO) -> Element<super::Message> {
        let mut main_column = Column::new()
            .spacing(10)
            .padding(Padding {
                top: 10.0,
                bottom: 10.0,
                left: 20.0,
                right: 20.0,
            })
            .width(iced::Length::Fill);

        main_column = main_column.push(
            Text::new(format!("Preset: {}", preset.name))
                .size(30)
                .width(iced::Length::Fill),
        );

        main_column = main_column
            .push(TextInput::new("", &self.temp_dto.name).on_input(super::Message::NameEdit));

        main_column = main_column.push(Text::new("Temperature").size(20).width(iced::Length::Fill));

        main_column = main_column.push(
            Row::new()
                .spacing(10)
                .align_y(iced::alignment::Vertical::Center)
                .push(
                    Slider::new(
                        0.0..=2.0,
                        self.temp_dto.temperature,
                        super::Message::TemperatureEdit,
                    )
                    .step(0.05)
                    .width(iced::Length::Fill),
                )
                .push(
                    Text::new(format!("{:.2}", self.temp_dto.temperature))
                        .size(20)
                        .width(50),
                ),
        );

        main_column = main_column.push(Text::new("Max Tokens").size(20).width(iced::Length::Fill));

        main_column = main_column.push(
            Row::new()
                .spacing(10)
                .align_y(iced::alignment::Vertical::Center)
                .push(
                    Slider::new(
                        0..=131072,
                        self.temp_dto.max_tokens,
                        super::Message::MaxTokensEdit,
                    )
                    .width(iced::Length::Fill)
                    .step(128u32),
                )
                .push(
                    Text::new(format!("{}", self.temp_dto.max_tokens))
                        .size(20)
                        .width(50),
                ),
        );

        main_column = main_column.push(Text::new("Prompt").size(20).width(iced::Length::Fill));

        main_column = main_column.push(
            TextEditor::new(&self.temp_prompt)
                .on_action(super::Message::PromptEdit)
                .height(400.0),
        );

        Container::new(
            Column::new()
                .spacing(10)
                .padding(Padding {
                    top: 10.0,
                    bottom: 10.0,
                    left: 20.0,
                    right: 20.0,
                })
                .push(
                    Container::new(Scrollable::new(main_column).style(scrollable_style))
                        .height(iced::Length::Fill),
                )
                .push(
                    Row::new()
                        .align_y(iced::alignment::Vertical::Center)
                        .spacing(10)
                        .push(Button::new("Save").on_press(super::Message::CommitChanges))
                        .push(horizontal_space())
                        .push(if self.is_unsaved_changes() {
                            Text::new("Unsaved changes").style(text::danger).size(20)
                        } else {
                            Text::new("").size(20)
                        })
                        .push(
                            Button::new("Delete")
                                .on_press(super::Message::DeletePreset)
                                .view()
                                .style(button::danger),
                        ),
                ),
        )
        .into()
    }

    fn view_add_preset(&self) -> Element<super::Message> {
        let mut btn = Button::new(
            Text::new("Add Preset")
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center)
                .width(iced::Length::Fill),
        )
        .view()
        .width(iced::Length::Fill);

        let default_preset = PresetDTO::default();
        if !self.presets.iter().any(|(_, preset)| preset.is_similar(&default_preset)) {
            btn = btn.on_press(super::Message::AddPreset);
        }

        Container::new(btn).padding(5).into()
    }
}
