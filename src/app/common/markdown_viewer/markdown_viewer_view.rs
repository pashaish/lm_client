use iced::{
    Element, Font, Length, Padding, Theme, font, theme::Palette, widget::{Button, Column, Row, button::Style, keyed::column, span, text::Span, text_editor}
};
use url::Url;

use crate::{
    app::common::markdown_viewer::markdown_viewer_state::{MdItem, MdItemVarian}, overrides::{self, rich::rich_text}, theme::dark_theme::{self, dark_theme, dark_theme_pallete}
};

use super::MarkdownViewer;

pub(super) const BASE_TEXT_SIZE: u16 = 16;

pub(super) struct ViewContext {
    pub text_size: u16,
}

enum RenderAction<'a> {
    Span { content: Span<'a, super::Message> },
    StartTable { columns_count: usize },
    TableNextRow,
    TableNextCell,
}

impl MarkdownViewer {
    pub fn view(&self) -> Element<super::Message> {
        let mut column = iced::widget::Column::new().padding(10).spacing(10);

        let mut rich_spans: Vec<Span<super::Message>> = vec![];

        let mut columns_count = 0;
        let mut current_table_row = Row::new();
        let mut current_table_cell = Column::new();
        let mut in_table = false;

        for item in &self.md_items {
            let actions = self.view_md_item(item, &ViewContext {
                text_size: BASE_TEXT_SIZE
            });


            for action in actions {
                match action {
                    RenderAction::Span { content } => {
                        rich_spans.push(content);
                    }
                    RenderAction::StartTable { columns_count: count } => {
                        columns_count = count;
                        in_table = true;
                        log::error!("START_TABLE")
                    }
                    RenderAction::TableNextCell => {
                        current_table_cell = current_table_cell.push(rich_text(rich_spans.clone()));
                        rich_spans.clear();

                        current_table_row = current_table_row.push(current_table_cell);
                        current_table_cell = Column::new().width(Length::Fixed(70.0));
                        log::error!("ADD_CELL")
                    },
                    RenderAction::TableNextRow => {
                        column = column.push(current_table_row);
                        current_table_row = Row::new();
                        log::error!("ADD_ROW")
                    },
                }
            }

            if !in_table {
                column = column.push(rich_text(rich_spans.clone()));
                rich_spans.clear();
            }
        }

        column.into()
    }

    fn view_md_item(&self, item: &MdItem, state: &ViewContext) -> Vec<RenderAction> {
        match &item.variant {
            MdItemVarian::Heading { content, level } => {
                let mut result = vec![];
                for child in content {
                    result.extend(self.view_md_item(child, &ViewContext { text_size: state.text_size * (6 - level) }));
                }

                result
            }
            MdItemVarian::Table { cells } => {
                let mut result = vec![];

                result.push(RenderAction::StartTable { columns_count: cells.len() });

                for row in cells {
                    result.push(RenderAction::TableNextRow);

                    for cell in row {
                        result.push(RenderAction::TableNextCell);

                        result.extend(self.view_md_item(cell, state));
                    }
                }
                
                result
            }
            MdItemVarian::Chunks { items } => {
                let mut result = vec![];
                for child in items {
                    result.extend(self.view_md_item(child, state));
                }

                result
            }
            MdItemVarian::Text { content } => {
                let mut result = vec![];

                result.push(
                    RenderAction::Span {
                        content: iced::widget::span(content.clone())
                            .size(state.text_size)
                            .into()
                    }
                );

                return result;
            }
        }
    }
}
