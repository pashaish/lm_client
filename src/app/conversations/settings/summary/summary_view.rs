use framework::Context;
use iced::{
    Element,
    widget::{Checkbox, Column, Container, TextEditor},
};

use super::Summary;

impl Summary {
    pub fn view(&self, ctx: &Context) -> Element<super::Message> {
        let mut main_column = Column::new()
            .spacing(10)
            .width(iced::Length::Fill);

        main_column = main_column.push(
            Checkbox::new("Summary", self.conversation.summary_enabled)
                .on_toggle(super::Message::ToggleSummary),
        );

        if self.conversation.summary_enabled {
            main_column = main_column.push(
                self.summary_model_picker.view(ctx).map(super::Message::SummaryModelPicker),
            );

            let mut text_editor = TextEditor::new(&self.summary_content);

            if self.hand_editing {
                text_editor = text_editor.on_action(super::Message::UpdateSummaryContent);
            }

            main_column = main_column.push(text_editor);
            if self.conversation.summary_enabled {
                main_column = main_column.push(
                    Checkbox::new("Hand Editing", self.hand_editing)
                        .on_toggle(super::Message::UpdateHandEditing),
                );
            }
        }

        Container::new(main_column).into()
    }
}
