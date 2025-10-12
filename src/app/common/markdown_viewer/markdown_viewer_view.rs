use cached::proc_macro::cached;
use iced::{
    Border, Color, Element, Font, Length, Padding, Shadow, Theme, advanced::Widget, font::{self, Weight}, theme::{Palette, palette::Background}, widget::{
        Button, Column, Container, MouseArea, Row, horizontal_space,
        keyed::column,
        span,
        text::{LineHeight, Span},
        text_editor,
    }
};
use url::Url;
use cached::SizedCache;

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
struct TableRow {
    pub cells: Vec<Vec<RenderSpan>>,
}

#[derive(Clone, Default)]
struct RenderSpan {
    pub words: Vec<String>,
    pub size: Option<u16>,
    pub font: Font,
    pub color: Option<Color>,
    pub underline: bool,
}

enum RenderAction {
    Span(RenderSpan),
    ListLevel { level: usize },
    StartTable,
    TableNextRow,
    TableNextCell { header: bool },
}

pub struct MarkdownViewerRenderConfig {
    pub plain: bool,
}

impl MarkdownViewer {
    pub fn view<'a>(&'a self, config: &MarkdownViewerRenderConfig) -> Element<'a, super::Message> {
        let mut column = iced::widget::Column::new().padding(4).spacing(4);

        let mut in_table = false;

        for item in &self.md_items {
            let mut rich_spans: Vec<RenderSpan> = vec![];
            let mut headers: Vec<Vec<RenderSpan>> = vec![];
            let mut rows: Vec<TableRow> = vec![];
            let mut list_level = 0;

            let actions = self.view_md_item(
                config,
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
                    RenderAction::Span(span) => {
                        rich_spans.push(span);
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
                        self.form_line(config, header, list_level),
                        move |row: TableRow| {
                            if row.cells.is_empty() {
                                return self.form_line(config, vec![], list_level);
                            }

                            self.form_line(config, row.cells[i].clone(), list_level)
                        },
                    )
                    .width(Length::Fill)
                });

                let table: overrides::table::Table<super::Message> =
                    overrides::table::table(columns, rows.clone()).width(Length::Fill);

                column = column.push(table);
                in_table = false;
            } else {
                column = column.push(self.form_line(config, rich_spans.clone(), list_level));
                rich_spans.clear();
            }
        }

        MouseArea::new(column)
            .on_enter(super::Message::OnEnterMouse)
            .on_exit(super::Message::OnLeaveMouse)
            .into()
    }

    fn form_line<'a>(
        &self,
        config: &MarkdownViewerRenderConfig,
        mut spans: Vec<RenderSpan>,
        list_level: usize,
    ) -> Element<'a, super::Message> {
        if list_level > 0 {
            spans.insert(
                0,
                RenderSpan {
                    words: vec![" ".repeat(list_level), "- ".to_string()],
                    ..Default::default()
                },
            );
        }

        let mut row = iced::widget::Row::new();

        // text_editor(content)

        // TODO: Flat?
        for span in spans {
            let underline = span.underline;
            let heading_color = self.config.heading_color;
            let word_style = move |_: &Theme| iced::widget::container::Style {
                // border: Border { width: 1.0, color: Color::from_rgb(0.0, 1.0, 0.0), ..Default::default() },
                shadow: if underline {
                    Shadow {
                        color: heading_color.clone(),
                        offset: iced::Vector::new(0.0, 2.0),
                        ..Default::default()
                    }
                } else {
                    Shadow::default()
                },
                ..Default::default()
            };

            for word in span.words.into_iter() {
                let text = iced::widget::text(word)
                    .size(span.size.unwrap_or(BASE_TEXT_SIZE))
                    .font(span.font);

                row = row.push(
                    Container::new(text.color(span.color.unwrap_or(Color::WHITE)))
                        .style(word_style),
                )
            }
        }

        row.wrap().into()
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

    fn view_md_item<'a>(
        &'a self,
        config: &MarkdownViewerRenderConfig,
        item: &'a MdItem,
        state: &ViewContext,
    ) -> Vec<RenderAction> {
        match &item.variant {
            MdItemVariant::Heading { content, level } => self.nesting(
                config,
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
                    config,
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
                        result.extend(self.view_md_item(config, cell, state));
                        result.push(RenderAction::TableNextCell {
                            header: row_index == 0,
                        });
                    }
                    result.push(RenderAction::TableNextRow);
                }

                result
            }
            MdItemVariant::Chunks { items } => self.nesting(config, items, state),
            MdItemVariant::Text { content } => {
                let mut result = vec![];

                let mut font = Font::default();

                if state.bold {
                    font.weight = Weight::ExtraBold;
                }

                if state.italic {
                    font.style = font::Style::Italic;
                }

                font.family = font::Family::Monospace;

                result.push(RenderAction::Span(RenderSpan {
                    // words: split(content),
                    words: vec![content.to_string()],
                    size: Some(state.text_size),
                    font: font,
                    color: if state.heading_level > 0 {
                        Some(self.config.heading_color)
                    } else {
                        Some(Color::WHITE)
                    },
                    underline: state.heading_level > 0,
                }));

                return result;
            }
            MdItemVariant::Strong { content } => self.nesting(
                config,
                content,
                &ViewContext {
                    bold: true,
                    ..(*state)
                },
            ),
            MdItemVariant::Emphasis { content } => self.nesting(
                config,
                content,
                &ViewContext {
                    italic: true,
                    ..(*state)
                },
            ),
        }
    }

    fn nesting<'a>(
        &'a self,
        config: &MarkdownViewerRenderConfig,
        content: &'a [MdItem],
        state: &ViewContext,
    ) -> Vec<RenderAction> {
        let mut result = vec![];
        for child in content {
            result.extend(self.view_md_item(config, child, state));
        }

        result
    }
}
#[cached(
    ty = "SizedCache<String, Vec<String>>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}", txt) }"#
)]
fn split(txt: &str) -> Vec<String> {
    let mut result = vec!["".to_string()];

    for chr in txt.chars() {
        if chr.is_whitespace() {
            if !result.last().unwrap().trim().is_empty() {
                result.push("".to_string());
            }
        } else {
            if result.last().unwrap().trim().is_empty() {
                result.push("".to_string());
            }
        }

        result.last_mut().unwrap().push_str(&chr.to_string());
    }

    result
}
