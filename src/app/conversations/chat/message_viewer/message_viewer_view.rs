use framework::{Context, types::dto::MessageID};
use iced::{
    Color, Element, Length, Padding, Theme,
    keyboard::{Key, key::Named},
    widget::{
        Column, Container, Row, Text, Tooltip, container, horizontal_space,
        text_editor::{self, Binding, KeyPress},
    },
};

use crate::{
    app::common::markdown_viewer::{self, MarkdownViewer, MarkdownViewerRenderConfig},
    widgets::{
        collapsible,
        icon::{IconName, IconType},
        icon_button::IconButton,
    },
};

use super::{MessageViewer, message_viewer_state::SharedState};

impl MessageViewer {
    pub fn view<'a>(
        &'a self,
        state: &'a SharedState,
        ctx: &'a Context,
    ) -> Element<'a, super::Message> {
        let mut main_column = Column::new();

        let reasoning_string = self.reasoning.get_original().trim();
        if !reasoning_string.is_empty() {
            main_column = main_column
                .push(collapsible(
                    "Reasoning",
                    self.markdown_content::<'a>(
                        state,
                        super::Message::EditReasoning,
                        &self.reasoning,
                        super::Message::ReasoningUpdate,
                        &state.editing_tmp_reasoning,
                    ),
                    self.reasoning_expanded,
                    super::Message::ReasoningExpanded(!self.reasoning_expanded),
                ))
                .spacing(10);
        }

        main_column = main_column
            .push(self.markdown_content::<'a>(
                state,
                super::Message::EditContent,
                &self.content,
                super::Message::ContentUpdate,
                &state.editing_tmp_content,
            ))
            .spacing(10);

        main_column = main_column.push(self.used_chunks(ctx));

        main_column = if self.is_editing(state) {
            main_column.push(self.editing_controls())
        } else {
            main_column.push(self.message_controls())
        };

        let is_user_message = self.is_user_message();

        let mut container = Container::new(main_column);

        container = container.width(Length::Fill);

        container
            .padding(10)
            .id(self.id.clone())
            .padding(Padding {
                top: 10.0,
                bottom: 10.0,
                left: 14.0,
                right: 14.0,
            })
            .style(move |theme: &Theme| {
                let palette = theme.extended_palette();

                let background = if is_user_message {
                    palette.primary.base.color.scale_alpha(0.5)
                } else {
                    palette.background.base.color.scale_alpha(0.0)
                };

                container::Style {
                    background: Some(iced::Background::Color(background)),
                    ..Default::default()
                }
            })
            .into()
    }

    fn used_chunks(&self, ctx: &Context) -> Element<'_, super::Message> {
        let mut main_row = Row::new().spacing(10);

        for chunk in &self.message_dto.chunks {
            let chunk_dto = ctx.vector_service.get_chunk(
                self.message_dto.conversation_id,
                chunk.chunk_id,
                chunk.dimension,
                &chunk.embedding_model,
            );

            if chunk_dto.is_err() {
                continue;
            }

            let chunk_dto = chunk_dto.unwrap();

            if chunk_dto.is_none() {
                continue;
            }

            let chunk_dto = chunk_dto.unwrap();

            let file = ctx
                .vector_service
                .get_file(self.message_dto.conversation_id, chunk_dto.file_id)
                .expect("File not found");

            main_row = main_row.push(Tooltip::new(
                Text::new(file.file_name).style(|theme: &Theme| {
                    let palette = theme.extended_palette();
                    iced::widget::text::Style {
                        color: Some(palette.primary.base.color),
                    }
                }),
                Container::new(Text::new(format!(
                    "{}: {}\n{}",
                    file.embedding_model, file.dimension, chunk_dto.chunk
                )))
                .max_width(600.0)
                .max_height(400.0)
                .padding(10)
                .style(|theme: &Theme| {
                    let palette = theme.extended_palette();
                    container::Style {
                        background: Some(iced::Background::Color(palette.primary.base.color)),
                        ..Default::default()
                    }
                }),
                iced::widget::tooltip::Position::FollowCursor,
            ))
        }

        Container::new(main_row).into()
    }

    fn markdown_content<'a>(
        &'a self,
        editing_state: &'a SharedState,
        submit_editing_message: impl Fn(text_editor::Action) -> super::Message + 'a,
        content: &'a MarkdownViewer,
        update_message: impl Fn(markdown_viewer::Message) -> super::Message + 'a,
        editing_content: &'a text_editor::Content,
    ) -> Element<'a, super::Message> {
        if self.is_editing(editing_state) {
            return text_editor::TextEditor::new(editing_content)
                .on_action(submit_editing_message)
                .key_binding(|key| self.key_bindings(key))
                .into();
        }

        return content
            .view(&MarkdownViewerRenderConfig {
                plain: !self.visible,
            })
            .map(update_message);
    }

    fn message_controls(&self) -> Element<'_, super::Message> {
        let is_gathering_message = self.get_id() == MessageID::default();
        Row::new()
            .push(horizontal_space())
            .push(
                IconButton::new(IconType::Solid(IconName::Pencil), super::Message::StartEdit)
                    .disabled(is_gathering_message),
            )
            .push(
                IconButton::new(IconType::Solid(IconName::Trash), super::Message::Delete)
                    .disabled(is_gathering_message),
            )
            .into()
    }

    fn editing_controls(&self) -> Element<'_, super::Message> {
        Row::new()
            .push(horizontal_space())
            .push(IconButton::new(
                IconType::Solid(IconName::XMark),
                super::Message::CancelEdit,
            ))
            .push(IconButton::new(
                IconType::Solid(IconName::FloppyDisk),
                super::Message::SubmitEdit,
            ))
            .into()
    }

    fn key_bindings(&self, key: KeyPress) -> Option<Binding<super::Message>> {
        if key.status != text_editor::Status::Focused {
            return None;
        }

        if key.modifiers.shift() && key.key == Key::Named(Named::Enter) {
            return Some(Binding::Custom(super::Message::SubmitEdit));
        }

        Binding::from_key_press(key)
    }
}
