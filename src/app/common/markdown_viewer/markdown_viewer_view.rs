use iced::{Color, Element, Padding, Pixels, Theme, never, widget::{Button, button::Style, rich_text, span, text, text_editor}};
use markdown::mdast::Node;

use crate::app::common::markdown_viewer::markdown_viewer_state::MdNode;

use super::MarkdownViewer;

pub(super) const BASE_TEXT_SIZE: u32 = 21;

pub(super) struct ViewContext {
    pub text_size: u32,
}

impl MarkdownViewer {
    pub fn view(&self) -> Element<super::Message> {
        return self.node_router(&self.node, &ViewContext {
            text_size: BASE_TEXT_SIZE,
        });
    }

    pub(super) fn node_router<'a>(&self, node: &'a MdNode, context: &ViewContext) -> Element<'a, super::Message> {
        match node {
            MdNode::Root { children } => {
                let children: Vec<Element<super::Message>> = children.iter().map(|child| self.node_router(child, context)).collect();
                iced::widget::column(children).into()
            }
            MdNode::Paragraph { children } => {
                let children: Vec<Element<super::Message>> = children.iter().map(|child| self.node_router(child, context)).collect();
                iced::widget::row(children).into()
            }
            MdNode::Text { value } => {
                let mut row = iced::widget::row(vec![]);

                for char in value {
                    let rich_char = rich_text([
                        span(char.value.clone())
                            .size(Pixels::from(context.text_size))
                            .background_maybe(if self.id_is_selected(char.id) {
                                    Color::from_rgb(0.3, 0.3, 0.8).into()
                            } else {
                                None
                            })
                    ])
                        .on_link_click(never)
                        .width(iced::Length::Shrink)
                        .height(iced::Length::Shrink);

                    let mut area = iced::widget::MouseArea::new(rich_char)
                            .on_press(super::Message::StartSelection(char.id))
                            .on_release(super::Message::EndSelection);

                    if self.start_selection.is_some() && self.end_selection.is_none() {
                        area = area.on_move(|_| super::Message::TempEndSelection(char.id));
                    }

                    row = row.push(area);
                }
                
                row.into()
            }
            MdNode::Heading { level, children } => {
                let size = match level {
                    1 => context.text_size + 12,
                    2 => context.text_size + 8,
                    3 => context.text_size + 4,
                    4 => context.text_size + 2,
                    5 => context.text_size + 1,
                    _ => context.text_size,
                };

                let children: Vec<Element<super::Message>> = children.iter().map(|child| self.node_router(
                    child,
                    &ViewContext { text_size: size, ..*context }
                )).collect();
                iced::widget::row(children)
                    .into()
            }
        }
    }
}
