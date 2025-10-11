use iced::{
    Border, Element, Font, Length, Padding, Theme, font,
    theme::Palette,
    widget::{Button, Column, Container, Row, keyed::column, span, text::Span, text_editor},
};
use url::Url;

use crate::{
    app::common::markdown_viewer::markdown_viewer_state::{MdItem, MdItemVarian},
    overrides::{self, rich::rich_text},
    theme::dark_theme::{self, dark_theme, dark_theme_pallete},
};

use super::MarkdownViewer;

pub(super) const BASE_TEXT_SIZE: u16 = 16;

pub(super) struct ViewContext {
    pub text_size: u16,
}

#[derive(Clone)]
struct TableRow<'a> {
    pub cells: Vec<Vec<Span<'a, super::Message>>>,
}

enum RenderAction<'a> {
    Span { content: Span<'a, super::Message> },
    StartTable,
    TableNextRow,
    TableNextCell { header: bool },
}

impl MarkdownViewer {
    pub fn view(&self) -> Element<super::Message> {
        let mut column = iced::widget::Column::new().padding(10).spacing(10);

        let mut in_table = false;

        for item in &self.md_items {
            let mut rich_spans: Vec<Span<super::Message>> = vec![];
            let mut headers: Vec<Vec<Span<super::Message>>> = vec![];
            let mut rows: Vec<TableRow> = vec![];

            let actions = self.view_md_item(
                item,
                &ViewContext {
                    text_size: BASE_TEXT_SIZE,
                },
            );

            for action in actions {
                match action {
                    RenderAction::Span { content } => {
                        rich_spans.push(content);
                    }
                    RenderAction::StartTable => {
                        in_table = true;
                    }
                    RenderAction::TableNextCell { header } => {
                        if header {
                            headers.push(rich_spans.clone());
                        } else {
                            rows.last_mut().unwrap().cells.push(rich_spans.clone());
                        }

                        rich_spans.clear();
                    }
                    RenderAction::TableNextRow => {
                        rows.push(TableRow { cells: vec![] });
                    }
                }
            }

            if in_table {
                let columns = headers.into_iter().enumerate().map(|(i, header)| {
                    crate::overrides::table::column(
                        rich_text(header),
                        move |row: TableRow| {
                            if row.cells.is_empty() {
                                return rich_text(vec![]);
                            }

                            rich_text(row.cells[i].clone())
                        },
                    )
                });

                let table: overrides::table::Table<super::Message> =
                    overrides::table::table(columns, rows.clone())
                        .width(Length::Fill);

                column = column.push(table);
            } else {
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
                    result.extend(self.view_md_item(
                        child,
                        &ViewContext {
                            text_size: state.text_size * (6 - level),
                        },
                    ));
                }

                result
            }
            MdItemVarian::Table { cells } => {
                let mut result = vec![];

                result.push(RenderAction::StartTable);

                for (row_index, row) in cells.iter().enumerate() {
                    for cell in row {
                        result.extend(self.view_md_item(cell, state));
                        result.push(RenderAction::TableNextCell {
                            header: row_index == 0,
                        });
                    }
                    result.push(RenderAction::TableNextRow);
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

                result.push(RenderAction::Span {
                    content: iced::widget::span(content.clone())
                        .size(state.text_size)
                        .into(),
                });

                return result;
            }
        }
    }
}
