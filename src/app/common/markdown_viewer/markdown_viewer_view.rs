use iced::{Element, Font, Padding, Theme, font, theme::Palette, widget::{Button, Column, button::Style, span, text::Span, text_editor}};
use url::Url;

use crate::{app::common::markdown_viewer::markdown_viewer_state::MdSpan, overrides::{self, rich::rich_text}, theme::dark_theme::{self, dark_theme, dark_theme_pallete}};

use super::MarkdownViewer;

pub(super) const BASE_TEXT_SIZE: u16 = 16;

pub(super) struct ViewContext {
    pub text_size: u16,
}

enum RenderAction<'a> {
    Push(Span<'a, super::Message>),
    NewLine,

    TableHeader { columns: usize },
    TableCellPush,
    TableEnd,
}

impl MarkdownViewer {
    pub fn view(&self) -> Element<super::Message> {
        let mut column = iced::widget::Column::new()
            .padding(10)
            .spacing(10);

        let mut current_line = Vec::new();

        for span in &self.md_spans {
            match self.render_span(span) {
                RenderAction::Push(element) => {
                    current_line.push(element);
                },
                RenderAction::NewLine => {
                    if current_line.is_empty() {
                        continue;
                    }

                    column = column.push(rich_text(current_line));
                    current_line = Vec::new();
                }
                RenderAction::TableHeader { columns } => {

                }
 
                RenderAction::TableCellPush => {

                },
                RenderAction::TableEnd => {

                },
            }
        }

        column = column.push(rich_text(current_line));

        column.into()
    }

    fn render_span(
        &self,
        span: &MdSpan,
    ) -> RenderAction {
        match span {
            MdSpan::Text { content, heading_level, strong, emphasis } => {
                let text_size = match heading_level {
                    Some(pulldown_cmark::HeadingLevel::H1) => BASE_TEXT_SIZE + 24,
                    Some(pulldown_cmark::HeadingLevel::H2) => BASE_TEXT_SIZE + 20,
                    Some(pulldown_cmark::HeadingLevel::H3) => BASE_TEXT_SIZE + 16,
                    Some(pulldown_cmark::HeadingLevel::H4) => BASE_TEXT_SIZE + 12,
                    Some(pulldown_cmark::HeadingLevel::H5) => BASE_TEXT_SIZE + 8,
                    Some(pulldown_cmark::HeadingLevel::H6) => BASE_TEXT_SIZE + 4,
                    None => BASE_TEXT_SIZE,
                };

                let mut text = iced::widget::span(content.clone())
                    .size(text_size);

                let mut font = Font::default(); 

                if *strong {
                    font.weight = font::Weight::ExtraBold;
                }

                if *emphasis {
                    font.style = font::Style::Italic;
                }

                text = text.font(font);

                if heading_level.is_some() {
                    text = text.underline(true);
                    text = text.color(dark_theme_pallete().primary);
                }

                RenderAction::Push(text.into())
            }
            MdSpan::NewLine => {
                RenderAction::NewLine
            }
            MdSpan::TableHeader { columns } => {
                RenderAction::TableHeader { columns: *columns }
            }
            MdSpan::TableCellPush => {
                RenderAction::TableCellPush
            }
            MdSpan::TableEnd => {
                RenderAction::TableEnd
            }
        }
    }
}
