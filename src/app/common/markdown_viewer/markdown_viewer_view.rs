use iced::{Element, Padding, Theme, widget::{Button, button::Style, rich_text, span, text_editor}};
use markdown::mdast::Node;

use crate::app::common::markdown_viewer::markdown_viewer_state::MdNode;

use super::MarkdownViewer;

pub(super) const BASE_TEXT_SIZE: u16 = 21;

pub(super) struct ViewContext {
    pub text_size: u16,
}

impl MarkdownViewer {
    pub fn view(&self) -> Element<super::Message> {
        return Self::node_router(&self.node, &ViewContext {
            text_size: BASE_TEXT_SIZE,
        });
    }

    pub(super) fn node_router<'a>(node: &'a MdNode, context: &ViewContext) -> Element<'a, super::Message> {
        match node {
            MdNode::Root { children } => {
                let children: Vec<Element<super::Message>> = children.iter().map(|child| Self::node_router(child, context)).collect();
                iced::widget::column(children).into()
            }
            MdNode::Paragraph { children } => {
                let children: Vec<Element<super::Message>> = children.iter().map(|child| Self::node_router(child, context)).collect();
                iced::widget::row(children).into()
            }
            MdNode::Text { value } => {
                let mut row = iced::widget::row(vec![]);

                for char in value.chars() {
                    let rich_char = rich_text([
                        span(char.clone())
                            .size(context.text_size)
                    ])
                        .width(iced::Length::Shrink)
                        .height(iced::Length::Shrink);

                    row = row.push(
                        iced::widget::MouseArea::new(
                            rich_char
                        )
                            .on_press(super::Message::Nothing)
                    );
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

                let children: Vec<Element<super::Message>> = children.iter().map(|child| Self::node_router(
                    child,
                    &ViewContext { text_size: size, ..*context }
                )).collect();
                iced::widget::row(children)
                    .into()
            }
        }
    }
}
