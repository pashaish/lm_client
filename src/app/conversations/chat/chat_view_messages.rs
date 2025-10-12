use framework::Context;
use iced::{Element, Length, widget::Scrollable};

use crate::{overrides, theme::styles::scrollable_style};

use super::Chat;

impl Chat {
    pub(super) fn _view_messages<'a>(&'a self, ctx: &'a Context) -> iced::Element<'a, super::Message> {
        let mut main_column = iced::widget::Column::new();

        main_column = main_column.push(overrides::list::List::new(&self.list_messages_content, |i, message| {
            message
                .view(&self.shared_messages_state, ctx)
                .map(|m| super::Message::UpdateMessage(message.get_id(), m))
                .into()
        }));

        if let Some(ref gathering_message) = self.gathering_message {
            let gathering_message: Element<'_, super::Message> = gathering_message
                .view(&self.shared_messages_state, ctx)
                .map(super::Message::UpdateGatheringMessage);

            main_column = main_column.push(gathering_message);
        }

        iced::widget::Container::new(
            Scrollable::new(main_column)
                .style(scrollable_style)
                .anchor_bottom()
                .on_scroll(super::Message::OnScrollMessageList)
                .height(Length::Fill),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }


    pub(super) fn view_messages<'a>(&'a self, ctx: &'a Context) -> iced::Element<'a, super::Message> {
        let mut main_column = iced::widget::Column::new()
            .spacing(10)
            .clip(true)
            .width(Length::Fill);

        let sorted_messages = self.get_sorted_messages();

        for message in sorted_messages {
            let message: Element<'_, super::Message> = message
                .view(&self.shared_messages_state, ctx)
                .map(|m| super::Message::UpdateMessage(message.get_id(), m));

            main_column = main_column.push(message);
        }

        if let Some(ref gathering_message) = self.gathering_message {
            let gathering_message: Element<'_, super::Message> = gathering_message
                .view(&self.shared_messages_state, ctx)
                .map(super::Message::UpdateGatheringMessage);

            main_column = main_column.push(gathering_message);
        }

        iced::widget::Container::new(
            Scrollable::new(main_column)
                .style(scrollable_style)
                .anchor_bottom()
                .on_scroll(super::Message::OnScrollMessageList)
                .height(Length::Fill),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}
