use iced::{
    Border, Element, Font, Length, Padding, Theme,
    font::{self, Weight},
    theme::Palette,
    widget::{Button, Column, Container, Row, keyed::column, span, text::Span, text_editor},
};
use url::Url;

use crate::{
    app::common::markdown_viewer::markdown_viewer_state::{MdItem, MdItemVariant},
    overrides::{self, rich::rich_text},
    theme::dark_theme::{self, dark_theme, dark_theme_pallete},
};

use super::MarkdownViewer;

pub(super) const BASE_TEXT_SIZE: u16 = 16;

pub(super) struct ViewContext {
    pub text_size: u16,
    pub bold: bool,
    pub italic: bool,
    pub list_level: usize,
    pub heading_level: u16,
}

#[derive(Clone)]
struct TableRow<'a> {
    pub cells: Vec<Vec<Span<'a, super::Message>>>,
}

enum RenderAction<'a> {
    Span { content: Span<'a, super::Message> },
    ListLevel { level: usize },
    StartTable,
    TableNextRow,
    TableNextCell { header: bool },
}

impl MarkdownViewer {
    pub fn view<'a>(&'a self) -> Element<'a, super::Message> {
        let mut column = iced::widget::Column::new().padding(10).spacing(10);

        let mut in_table = false;

        for item in &self.md_items {
            let mut rich_spans: Vec<Span<super::Message>> = vec![];
            let mut headers: Vec<Vec<Span<super::Message>>> = vec![];
            let mut rows: Vec<TableRow> = vec![];
            let mut list_level = 0;

            let form_line = |mut spans: Vec<Span<'a, super::Message>>,
                             list_level: usize|
             -> Element<'a, super::Message> {
                if list_level > 0 {
                    spans.insert(0, span("- "));
                    spans.insert(0, span("  ".repeat(list_level)));
                }

                rich_text(spans.clone()).into()
            };

            let actions = self.view_md_item(
                item,
                &ViewContext {
                    text_size: BASE_TEXT_SIZE,
                    bold: false,
                    italic: false,
                    list_level: 0,
                    heading_level: 0,
                },
            );

            for action in actions {
                match action {
                    RenderAction::ListLevel { level } => {
                        list_level = level;
                    }
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
                if let Some(last_row) = rows.last() {
                    if last_row.cells.is_empty() {
                        rows.pop();
                    }
                }

                let columns = headers.into_iter().enumerate().map(|(i, header)| {
                    crate::overrides::table::column(
                        form_line(header, list_level),
                        move |row: TableRow| {
                            if row.cells.is_empty() {
                                return form_line(vec![], list_level);
                            }

                            form_line(row.cells[i].clone(), list_level)
                        },
                    )
                    .width(Length::Fill)
                });

                let table: overrides::table::Table<super::Message> =
                    overrides::table::table(columns, rows.clone()).width(Length::Fill);

                column = column.push(table);
                in_table = false;
            } else {
                column = column.push(form_line(rich_spans.clone(), list_level));
                rich_spans.clear();
            }
        }

        column.into()
    }

    fn level_to_text_size(level: u16) -> u16 {
        match level {
            1 => (BASE_TEXT_SIZE as f32 * 2.5) as u16,
            2 => (BASE_TEXT_SIZE as f32 * 2.0) as u16,
            3 => (BASE_TEXT_SIZE as f32 * 1.5) as u16,
            4 => (BASE_TEXT_SIZE as f32 * 1.3) as u16,
            5 => (BASE_TEXT_SIZE as f32 * 1.1) as u16,
            6 => (BASE_TEXT_SIZE as f32 * 1.0) as u16,
            _ => panic!("Invalid heading level"),
        }
    }

    fn view_md_item(&self, item: &MdItem, state: &ViewContext) -> Vec<RenderAction> {
        match &item.variant {
            MdItemVariant::Heading { content, level } => self.nesting(
                content,
                &ViewContext {
                    text_size: Self::level_to_text_size(*level),
                    heading_level: *level,
                    ..(*state)
                },
            ),
            MdItemVariant::Item { content } => {
                let mut result = vec![];

                result.push(RenderAction::ListLevel {
                    level: state.list_level + 1,
                });

                result.extend(self.nesting(
                    content,
                    &ViewContext {
                        list_level: state.list_level + 1,
                        ..(*state)
                    },
                ));

                result
            }
            MdItemVariant::Table { cells } => {
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
            MdItemVariant::Chunks { items } => self.nesting(items, state),
            MdItemVariant::Text { content } => {
                let mut result = vec![];

                let content = content.clone();

                let mut span = iced::widget::span(content).size(state.text_size).font({
                    let mut font = Font::default();

                    if state.bold {
                        font.weight = Weight::ExtraBold;
                    }

                    if state.italic {
                        font.style = font::Style::Italic;
                    }

                    font
                });


                if state.heading_level > 0 {
                    span = span.underline(true);
                    span = span.color(self.config.heading_color);
                }

                result.push(RenderAction::Span {
                    content: span.into(),
                });

                return result;
            }
            MdItemVariant::Strong { content } => self.nesting(
                content,
                &ViewContext {
                    bold: true,
                    ..(*state)
                },
            ),
            MdItemVariant::Emphasis { content } => self.nesting(
                content,
                &ViewContext {
                    italic: true,
                    ..(*state)
                },
            ),
        }
    }

    fn nesting(&self, content: &[MdItem], state: &ViewContext) -> Vec<RenderAction> {
        let mut result = vec![];
        for child in content {
            result.extend(self.view_md_item(child, state));
        }

        result
    }
}
