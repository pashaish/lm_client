use iced::{
    Element, Length, Padding, Theme,
    widget::{Container, MouseArea, Row, TextInput, container::Style, horizontal_space},
};

use crate::widgets::{
    context_menu::Action,
    context_menu::ContextMenu,
    icon::{Icon, IconName, IconType},
};

use super::{
    NodeAction, TreeNode,
    tree_node_state::{Content, SharedState},
};

impl TreeNode {
    pub fn view(&self, state: &SharedState) -> Element<super::Message> {
        let title = if self.is_renaming_process(state) {
            self.rename_process_title(state)
        } else {
            self.clone().title(state)
        };

        let children = self.children(state);

        let mut main_column = iced::widget::Column::new();

        if !self.is_root() {
            main_column = main_column.push(title);
        }

        if let Some(children) = children {
            if let Content::Folder(_) = &self.content {
                if self.is_expanded(state) || self.is_root() {
                    main_column = main_column.push(
                        Container::new(children)
                            .style(|theme: &Theme| {
                                let shadow = if self.is_root() {
                                    iced::Shadow::default()
                                } else {
                                    iced::Shadow {
                                        color: theme
                                            .extended_palette()
                                            .secondary
                                            .weak
                                            .color
                                            .scale_alpha(0.2),
                                        offset: iced::Vector::new(-1.0, 0.0),
                                        blur_radius: 0.0,
                                    }
                                };

                                Style {
                                    shadow,
                                    ..Default::default()
                                }
                            })
                            .padding(Padding {
                                top: 0.0,
                                bottom: 0.0,
                                left: if self.is_root() { 0.0 } else { 10.0 },
                                right: 0.0,
                            }),
                    );
                }
            }
        }

        Container::new(main_column).into()
    }

    pub fn rename_process_title<'a>(&'a self, state: &SharedState) -> Element<'a, super::Message> {
        TextInput::new("Name", &self.temp_name(state))
            .on_input(|val| super::Message::NodeAction(self.id, NodeAction::RenameProcess(val)))
            .on_submit(super::Message::NodeAction(
                self.id,
                NodeAction::RenameStartSave,
            ))
            .id(self.focus_id.clone())
            .into()
    }

    pub fn title<'a>(self, state: &SharedState) -> Element<'a, super::Message> {
        let mut main_row = Row::new()
            .spacing(10)
            .padding(Padding {
                top: 0.0,
                bottom: 0.0,
                left: 5.0,
                right: 5.0,
            })
            .align_y(iced::alignment::Alignment::Center);

        let icn = self.get_icon(state);

        main_row = main_row.push(icn);
        main_row = main_row.push(iced::widget::text(self.name.clone()).size(16));
        main_row = main_row.push(horizontal_space());

        let is_hover = self.is_hover(state);
        let is_pressed = self.is_pressed(state);
        let is_selected = self.is_selected(state);

        let ctx_menu = ContextMenu::new(
            MouseArea::new(
                Container::new(main_row)
                    .style(Self::context_menu_style(is_hover, is_pressed, is_selected))
                    .padding(Padding {
                        top: 2.0,
                        bottom: 2.0,
                        left: 5.0,
                        right: 5.0,
                    }),
            )
            .on_press(super::Message::NodeAction(self.id, NodeAction::Press))
            .on_release(super::Message::NodeAction(
                self.id,
                if self.is_chat() {
                    NodeAction::ReleaseChat
                } else {
                    NodeAction::ReleaseFolder
                },
            ))
            .on_move(move |point| {
                super::Message::NodeAction(self.id, NodeAction::MouseMoved(point))
            })
            .on_enter(super::Message::NodeAction(self.id, NodeAction::Hover(true)))
            .on_exit(super::Message::NodeAction(
                self.id,
                NodeAction::Hover(false),
            ))
            .interaction(iced::mouse::Interaction::Pointer)
            .into(),
        );

        let ctx_menu = match self.content {
            Content::Folder(_) => ctx_menu
                .action(self.create_action(
                    "New Folder",
                    super::NodeAction::StartFolderCreate,
                    IconType::Solid(IconName::FolderPlus),
                ))
                .action(self.create_action(
                    "New Chat",
                    super::NodeAction::StartConversationCreate,
                    IconType::Solid(IconName::Comments),
                ))
                .action(self.create_action(
                    "Rename",
                    super::NodeAction::StartRename,
                    IconType::Solid(IconName::Pencil),
                ))
                .action(self.create_action(
                    "Delete",
                    super::NodeAction::StartDelete,
                    IconType::Solid(IconName::Trash),
                )),

            Content::Chat(_) => ctx_menu
                .action(self.create_action(
                    "Rename",
                    super::NodeAction::StartRename,
                    IconType::Solid(IconName::Pencil),
                ))
                .action(self.create_action(
                    "Delete",
                    super::NodeAction::StartDelete,
                    IconType::Solid(IconName::Trash),
                )),
            Content::Loading => ctx_menu,
        };

        ctx_menu.into()
    }

    fn context_menu_style(
        is_hover: bool,
        is_pressed: bool,
        is_selected: bool,
    ) -> impl Fn(&Theme) -> Style {
        move |theme: &Theme| {
            let palette = theme.extended_palette();

            let background = if is_hover {
                Some(iced::Background::Color(
                    palette.secondary.weak.color.scale_alpha(0.1),
                ))
            } else if is_selected {
                Some(iced::Background::Color(
                    palette.secondary.base.color.scale_alpha(0.2),
                ))
            } else if is_pressed {
                Some(iced::Background::Color(
                    palette.secondary.base.color.scale_alpha(0.4),
                ))
            } else {
                None
            };

            Style {
                text_color: Some(palette.secondary.base.text),
                background,
                ..Default::default()
            }
        }
    }

    const fn create_action<'a>(
        &self,
        name: &'a str,
        node_action: NodeAction,
        icon: IconType,
    ) -> Action<'a, super::Message> {
        Action::new(name, super::Message::NodeAction(self.id, node_action)).icon(icon)
    }

    fn get_icon<'a>(&self, state: &SharedState) -> Element<'a, super::Message> {
        let icon = match &self.content.clone() {
            Content::Folder(_) => {
                if self.is_expanded(state) {
                    Icon::new(IconType::Solid(IconName::ChevronDown))
                        .view()
                        .size(12.0)
                } else {
                    Icon::new(IconType::Solid(IconName::ChevronRight))
                        .view()
                        .size(12.0)
                }
            }
            Content::Chat(_) => Icon::new(IconType::Solid(IconName::Comments))
                .view()
                .size(16.0),
            Content::Loading => Icon::new(IconType::Solid(IconName::Spinner))
                .view()
                .size(16.0),
        };

        Container::new(icon).into()
    }

    fn children(&self, state: &SharedState) -> Option<Element<'_, super::Message>> {
        if let Content::Folder(descriptor) = &self.content {
            let mut column = iced::widget::Column::new();
            column = column.push(self.insert_place(state, 0));

            for (idx, child) in descriptor.children.iter().enumerate() {
                column = column.push(child.view(state));
                column = column.push(self.insert_place(state, idx + 1));
            }

            Some(column.into())
        } else {
            None
        }
    }

    fn insert_place(&self, state: &SharedState, index: usize) -> Element<'_, super::Message> {
        let insert_place_index = self.get_insert_place_index(state);
        let dragged = state.dragged;
        let is_hovered = insert_place_index == Some(index);
        let height = if state.dragged.is_some() {
            if is_hovered { 10.0 } else { 5.0 }
        } else {
            2.5
        };

        let element = MouseArea::new(
            Container::new("")
                .style(move |theme: &Theme| {
                    let palette = theme.extended_palette();

                    let background = if dragged.is_some() && is_hovered {
                        Some(iced::Background::Color(
                            palette.secondary.strong.color.scale_alpha(0.1),
                        ))
                    } else {
                        None
                    };

                    Style {
                        text_color: Some(palette.secondary.base.text),
                        background,
                        ..Default::default()
                    }
                })
                .height(height)
                .width(Length::Fill),
        )
        .on_enter(super::Message::NodeAction(
            self.id,
            NodeAction::HoverInsertPlace(Some(index)),
        ))
        .on_exit(super::Message::NodeAction(
            self.id,
            NodeAction::HoverInsertPlace(None),
        ));

        element.into()
    }
}
