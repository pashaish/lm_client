use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

use iced::{Color, Element, Task, widget::text_editor};
use pulldown_cmark::HeadingLevel;
use url::Url;

use crate::{
    app::common::markdown_viewer::{
        markdown_viewer_update::ParsingState,
        markdown_viewer_view::{BASE_TEXT_SIZE, ViewContext},
    },
    overrides, theme::dark_theme::dark_theme_pallete,
};

#[derive(Debug, Clone)]
pub enum Message {
    Update(String),

    StartSelection(usize),
    EndSelection(usize),

    LinkClicked(Url),
}

#[derive(Debug, Clone)]
pub struct MdItem {
    pub variant: MdItemVarian,
    pub is_completed: bool,
}

impl MdItem {
    pub fn push_text(&mut self, str: &str) {
        let text_item = MdItem {
            variant: MdItemVarian::Text {
                content: str.to_string(),
            },
            is_completed: true,
        };

        self.push(&text_item);
    }

    pub fn push(&mut self, item: &MdItem) {
        match &mut self.variant {
            MdItemVarian::Emphasis { content } |
            MdItemVarian::Strong { content } |
            MdItemVarian::Heading { content, .. } => {
                content.push(item.clone());
            }
            MdItemVarian::Table { cells } => {
                if let Some(last_row) = cells.last_mut() {
                    last_row.push(item.clone());
                } else {
                    cells.push(vec![item.clone()]);
                }
            }
            MdItemVarian::Text { content: _ } => {
                panic!("Wrong Insert")
            }
            MdItemVarian::Chunks { items } => {
                items.push(item.clone());
            }
            MdItemVarian::Item { content } => {
                content.push(item.clone());
            }
        }
    }

    pub fn last_child_mut(&mut self) -> Option<&mut MdItem> {
        match &mut self.variant {
            MdItemVarian::Heading { content, level } => content.last_mut(),
            MdItemVarian::Table { cells } => cells.last_mut().and_then(|l| l.last_mut()),
            MdItemVarian::Text { content } => None,
            MdItemVarian::Chunks { items } => items.last_mut(),
            MdItemVarian::Strong { content } => content.last_mut(),
            MdItemVarian::Emphasis { content } => content.last_mut(),
            MdItemVarian::Item { content } => content.last_mut(),
        }
    }

    pub fn last_child(&self) -> Option<&MdItem> {
        match &self.variant {
            MdItemVarian::Heading { content, level } => content.last(),
            MdItemVarian::Table { cells } => cells.last().and_then(|l| l.last()),
            MdItemVarian::Text { content } => None,
            MdItemVarian::Chunks { items } => items.last(),
            MdItemVarian::Strong { content } => content.last(),
            MdItemVarian::Emphasis { content } => content.last(),
            MdItemVarian::Item { content } => content.last(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MdItemVarian {
    Chunks { items: Vec<MdItem> },
    Text { content: String },
    Heading { content: Vec<MdItem>, level: u16 },
    Table { cells: Vec<Vec<MdItem>> },
    Strong { content: Vec<MdItem> },
    Emphasis { content: Vec<MdItem> },
    Item { content: Vec<MdItem> },
}

#[derive(Clone)]
pub struct StyleConfig {
    pub heading_color: Color,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            heading_color: dark_theme_pallete().text,
        }
    }
}

pub struct MarkdownViewer {
    pub(super) original: String,
    pub(super) md_items: Vec<MdItem>,

    pub(super) config: StyleConfig,
}

impl Clone for MarkdownViewer {
    fn clone(&self) -> Self {
        let orig_clone = self.original.clone();
        Self {
            original: orig_clone,
            md_items: self.md_items.clone(),

            config: self.config.clone(),
        }
    }
}

impl Debug for MarkdownViewer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownViewer")
            .field("original", &self.original)
            .field("md_spans", &self.md_items)
            .finish()
    }
}

impl MarkdownViewer {
    pub fn new(original: &str, config: StyleConfig) -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        tasks.push(Task::done(Message::Update(original.to_string())));

        (
            Self {
                original: "".to_string(),
                md_items: vec![],
                
                config,
            },
            iced::Task::batch(tasks),
        )
    }

    pub fn get_original(&self) -> &str {
        &self.original
    }
}
