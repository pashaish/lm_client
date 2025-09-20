use std::ops::RangeInclusive;

use framework::Context;
use iced::{
    keyboard::key::{Code, Physical}, widget::{horizontal_space, text_editor::{Binding, KeyPress}, Column, Container, ProgressBar, Row, Text, TextEditor}, Element, Length
};

use crate::widgets::button::Button;

use super::Chat;

impl Chat {
    pub(super) fn view_texteditor(&self, _ctx: &Context) -> Element<'_, super::Message> {
        let mut main_column = iced::widget::Column::new()
            .spacing(5)
            .padding(5)
            .width(Length::Fill);

        let is_available = !self.gathering_message_process && !self.loading_file;

        let mut text_editor = TextEditor::new(&self.text_editor_content)
            .id(self.text_editor_id.clone());

        if is_available && !self.is_need_generate {
            text_editor = text_editor
                .key_binding(Self::key_bindings)
                .on_action(super::Message::UpdateTextEditor);
        }

        main_column = main_column.push(text_editor);

        if self.gathering_message_aborter.is_some() {
            let stop_button = Button::new(
                iced::widget::Text::new("Stop")
                    .align_x(iced::alignment::Horizontal::Center)
                    .align_y(iced::alignment::Vertical::Center)
                    .width(Length::Fill),
            )
            .view()
            .on_press(super::Message::StopMessageLoading)
            .padding(5)
            .width(Length::Fill);
    
            main_column = main_column.push(
                Row::new()
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(10)
                    .spacing(10)
                    .push(horizontal_space())
                    .push(stop_button),
            );
        } else {
            let send_btn_text = if self.is_need_generate {
                "Generate"
            } else {
                "Send"
            };

            let mut send_button = Button::new(
                iced::widget::Text::new(send_btn_text)
                    .align_x(iced::alignment::Horizontal::Center)
                    .align_y(iced::alignment::Vertical::Center)
                    .width(Length::Fill),
            )
            .view()
            .padding(5)
            .width(Length::Fill);
    
            if is_available {
                send_button = send_button.on_press(super::Message::SendMessage);
            }
    
            main_column = main_column.push(
                Row::new()
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(10)
                    .spacing(10)
                    .push(horizontal_space())
                    .push(send_button),
            );
        }

        let main_column = if let Some(bar) = self.view_progress_bar_file_loading() {
            main_column.push(bar)
        } else {
            main_column
        };

        Container::new(main_column).into()
    }

    fn key_bindings(key_press: KeyPress) -> Option<Binding<super::Message>> {
        match key_press.clone().physical_key {
            Physical::Code(Code::Enter) if key_press.modifiers.shift() => Some(Binding::Enter),
            Physical::Code(Code::Enter) => Some(Binding::Custom(super::Message::SendMessage)),
            _ => Binding::from_key_press(key_press),
        }
    }

    fn view_progress_bar_file_loading(&self) -> Option<Element<super::Message>> {
        if let Some((label, range, value)) = self.loading_progress.clone() {
            return Some(Container::new(
                Column::new()
                    .padding(5)
                    .spacing(5)
                    .push(Text::new(label).style(iced::widget::text::secondary))
                    .push(
                        ProgressBar::new(RangeInclusive::new(range.start, range.end), value)
                            .height(5)
                            .width(Length::Fill)
                    )
            ).into());
        }
        
        None
    }
}
