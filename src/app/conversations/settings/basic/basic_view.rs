use std::ops::RangeInclusive;

use framework::{types::dto::PresetDTO, Context};
use iced::{widget::{text_editor, Column, Container, PickList, Row, Slider, Text, TextEditor, TextInput}, Element};

use super::Basic;

impl Basic {
    pub fn view(&self, ctx: &Context) -> Element<super::Message> {
        let mut main_column = Column::new()
            .spacing(10)
            .padding(10)
            .width(iced::Length::Fill);

        main_column = main_column
            .push(self.field_setting("Name", self.name_input()))
            .push(self.field_setting("Presets", self.presets_selector()))
            .push(self.field_setting(
                "Max Messages",
                self.slider_input(
                    RangeInclusive::new(1, 100),
                    self.conversation.max_messages,
                    super::Message::ChangeMaxMessages,
                ),
            ))
            .push(self.model_picker.view(ctx).map(super::Message::ModelPicker))
            .push(
                self.field_setting("Conversation Prompt", self.prompt_input())
            );

        main_column.into()
    }

    fn prompt_input(&self) -> Element<super::Message> {
        let editor = TextEditor::new(&self.preset_content)
            .on_action(super::Message::PresetEdit);

        editor.into()
    }

    fn name_input(&self) -> Element<super::Message> {
        let mut input = TextInput::new("", &self.temp_name);

        if !self.save_process {
            input = input
                .on_input(super::Message::InputName)
                .on_submit(super::Message::SubmitName);
        }

        input.into()
    }

    fn presets_selector(&self) -> Element<super::Message> {
        let mut presets = self.presets.clone();

        let none_preset = PresetDTO {
            id: 0,
            name: "None".to_string(),
            ..Default::default()
        };

        presets.insert(0, none_preset.clone());

        let selected_preset = self.selected_preset.clone().unwrap_or(none_preset);

        let pick_list = PickList::new(presets, Some(selected_preset), super::Message::SelectPreset)
            .width(iced::Length::Fill);

        Container::new(pick_list).into()
    }

    fn field_setting<'a>(
        &'a self,
        field_name: &'a str,
        field_element: Element<'a, super::Message>,
    ) -> Element<'a, super::Message> {
        Column::new()
            .spacing(10)
            .push(
                Text::new(field_name)
                    .style(iced::widget::text::secondary)
                    .align_x(iced::Alignment::Center)
                    .align_y(iced::Alignment::Center),
            )
            .push(field_element)
            .into()
    }

    fn slider_input(
        &self,
        range: RangeInclusive<i32>,
        value: usize,
        message: impl Fn(i32) -> super::Message + 'static,
    ) -> Element<super::Message> {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let value = value as i32;

        let slider = Slider::<i32, super::Message>::new(
            range,
            value,
            message,
        );

        Container::new(
            Row::new()
                .spacing(10)
                .width(iced::Length::Fill)
                .align_y(iced::Alignment::Center)
                .push(slider)
                .push(Text::new(value.to_string())),
        )
        .into()
    }
}
