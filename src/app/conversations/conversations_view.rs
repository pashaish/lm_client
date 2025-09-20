use framework::Context;
use iced::{
    Element,
    widget::{Container, PaneGrid, Row, pane_grid},
};

use super::{Conversations, conversations_state::Pane};

impl Conversations {
    pub fn view<'a>(&'a self, ctx: &'a Context) -> Element<'a, super::Message> {
        let mut main_row = Row::new();

        main_row = main_row.push(
            PaneGrid::new(&self.panes, |_, pane, _| {
                let content: Element<_> = match pane {
                    Pane::Folders => self.folders.view(ctx).map(super::Message::Folders),
                    Pane::Chat => {
                        self.get_chat().map_or_else(|| Container::new("Chat not selected")
                                .align_x(iced::Alignment::Center)
                                .align_y(iced::Alignment::Center)
                                .into(), |chat| chat.view(self.settings_expanded(), ctx).map(|m| {
                                super::Message::Chat(
                                    self.current_chat_id.expect("Chat ID is None"),
                                    m,
                                )
                            }))
                    }
                    Pane::Settings => {
                        if let Some(ref settings) = self.settings {
                            return settings.view(ctx).map(super::Message::Settings).into();
                        }

                        return Container::new("Settings not available")
                            .align_x(iced::Alignment::Center)
                            .align_y(iced::Alignment::Center)
                            .into();
                    },
                };

                pane_grid::Content::new(content)
            })
            .on_resize(5, super::Message::Resize),
        );

        Container::new(main_row).into()
    }
}
