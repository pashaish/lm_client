use framework::Context;
use iced::{
    Element, Padding, Size,
    widget::{
        Column, MouseArea, Row, Scrollable, container, space,
    },
};

use crate::{
    theme::styles::{self},
    widgets::{
        dragging_placeholder::DraggingPlaceholder,
        icon::{IconName, IconType},
        icon_button::IconButton,
    },
};

use super::{Folders, tree_node::SharedState};

impl Folders {
    pub fn view(&self, _ctx: &Context) -> Element<super::Message> {
        let mut main_column = Column::new().width(iced::Length::Fill).padding(Padding {
            bottom: 5.0,
            ..Default::default()
        });

        main_column = main_column.push(
            Row::new()
                .push(space::horizontal())
                .push(IconButton::new(
                    IconType::Solid(IconName::Plus),
                    super::Message::CreateChat,
                ))
                .push(IconButton::new(
                    IconType::Solid(IconName::FolderPlus),
                    super::Message::CreateFolder,
                )),
        );
        main_column = main_column.push(
            self.root_folder
                .view(&self.shared_state)
                .map(super::Message::TreeNode),
        );

        main_column = main_column
            .push(
                MouseArea::new(space::vertical().width(iced::Length::Fill).height(100))
                    .on_release(super::Message::ReleaseFreeArea),
            );

        if let Some(dragged_id) = self.shared_state.dragged {
            let item = self
                .root_folder
                .find_child(dragged_id)
                .expect("Item should be present in the tree");

            let placeholder = DraggingPlaceholder::new(
                move || {
                    item.clone()
                        .title(&SharedState::new())
                        .map(super::Message::TreeNode)
                },
                true,
                Size::new(200.0, 100.0),
            );

            main_column = main_column.push(placeholder);
        }

        iced::widget::Container::new(
            Scrollable::new(main_column)
                .style(styles::scrollable_style)
        )
            .style(|theme| container::Style {
                shadow: styles::fake_oneside_border(theme, &styles::Side::Right),
                ..Default::default()
            })
            .padding(Padding {
                right: 5.0,
                left: 0.0,
                ..Default::default()
            })
            .into()
    }
}
