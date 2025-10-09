use std::{collections::HashMap, ops::RangeInclusive};

use iced::{widget::{button, Column, Container, Row, Slider, Text}, Element};
use framework::{types::dto::RagFileDTO, Context};

use crate::widgets::{button::Button, 
    // icon::{IconName, IconType}, icon_button::IconButton
};

use super::Rag;

impl Rag {
    pub fn view(&self, ctx: &Context) -> Element<super::Message> {
        let main_column = Column::new()
            .spacing(10)
            .push(
                self.field_setting("Max Chunk Size", self.slider_input(
                    RangeInclusive::new(128, 2048),
                    self.conversation.rag_chunk_size,
                    super::Message::ChangeChunkSize
                )),
            )
            .push(
                self.field_setting("Chunks Count", self.slider_input(
                    RangeInclusive::new(1, 32),
                    self.conversation.rag_chunks_count,
                    super::Message::ChangeChunksCount
                )),
            )
            .push(self.model_picker.view(ctx).map(super::Message::ModelPicker))
            .push(self.rag_files());

        main_column.into()
    }

    fn rag_files(&self) -> Element<super::Message> {
        let mut main_column = Column::new()
            .spacing(10)
            .width(iced::Length::Fill)
            .align_x(iced::Alignment::Start);

        let mut rag_fiels_grouped_by_model: HashMap<String, Vec<RagFileDTO>> = HashMap::new();

        let mut group_names = vec![]; 
        for rag_file in &self.rag_files {
            let model = rag_file.embedding_model.clone();
            let dimension = rag_file.dimension;

            let group_name = format!("{model} / {dimension}");

            if let Some(group) = rag_fiels_grouped_by_model.get_mut(&group_name) {
                group.push(rag_file.clone());
            } else {
                group_names.push(group_name.clone());
                rag_fiels_grouped_by_model.insert(group_name, vec![rag_file.clone()]);
            }
        }

        group_names.sort();

        for model_name in group_names {
            let rag_files = rag_fiels_grouped_by_model
                .get(&model_name)
                .expect("Failed to get rag files");

            main_column = main_column.push(
                Text::new(model_name)
                    .size(12)
                    .style(iced::widget::text::secondary)
                    .width(iced::Length::Fill)
                    .align_x(iced::Alignment::Start)
                    .align_y(iced::Alignment::Center),
            );

            for rag_file in rag_files {
                let rag_file_name = Text::new(rag_file.file_name.clone())
                    .width(iced::Length::Fill)
                    .align_x(iced::Alignment::Start)
                    .align_y(iced::Alignment::Center);

                main_column = main_column.push(Container::new(
                    Row::new()
                        .push(rag_file_name)
                // ?TODO: NEED UPDATE

                        .push(
                            Button::new(
                                Text::new("🗑️")
                                    .width(iced::Length::Fill)
                                    .align_x(iced::Alignment::Center)
                                    .align_y(iced::Alignment::Center),
                            )
                            .on_press(super::Message::StartDeletingRagFile(rag_file.id))
                            .view()
                            .style(button::danger)
                            .padding(5)
                        )
                        // .push(IconButton::new(
                        //     IconType::Solid(IconName::Trash),
                        //     super::Message::StartDeletingRagFile(rag_file.id),
                        // )),
                ));
            }
        }

        if self.loading_files_aborter.is_some() {
            main_column = main_column.push(
                Button::new(
                    Text::new("Cancel files loading...")
                        .width(iced::Length::Fill)
                        .align_x(iced::Alignment::Center)
                        .align_y(iced::Alignment::Center)
                    )
                    .on_press(super::Message::CancelLoadingFiles)
                    .view()
                    .style(button::danger)
                    .width(iced::Length::Fill)
                    .padding(5)
            );
        } else {
            main_column = main_column.push(
                Button::new(
                    Text::new("Load files")
                        .width(iced::Length::Fill)
                        .align_x(iced::Alignment::Center)
                        .align_y(iced::Alignment::Center)
                    )
                    .on_press(super::Message::StartLoadingFiles)
                    .view()
                    .width(iced::Length::Fill)
                    .padding(5)
            );
        }

        Container::new(main_column).into()
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
