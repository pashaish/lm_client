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

    OnEnterMouse,
    OnLeaveMouse,

    StartSelection(usize),
    EndSelection(usize),

    LinkClicked(Url),
}

#[derive(Debug, Clone)]
pub struct MdItem {
    pub variant: MdItemVariant,
    pub is_completed: bool,
}

impl MdItem {
    pub fn push_text(&mut self, str: &str) {
        let text_item = MdItem {
            variant: MdItemVariant::Text {
                content: str.to_string(),
            },
            is_completed: true,
        };

        self.push(&text_item);
    }

    pub fn push(&mut self, item: &MdItem) {
        match &mut self.variant {
            MdItemVariant::Emphasis { content } |
            MdItemVariant::Strong { content } |
            MdItemVariant::Heading { content, .. } => {
                content.push(item.clone());
            }
            MdItemVariant::Table { cells } => {
                if let Some(last_row) = cells.last_mut() {
                    last_row.push(item.clone());
                } else {
                    cells.push(vec![item.clone()]);
                }
            }
            MdItemVariant::Text { content: _ } => {
                panic!("Wrong Insert")
            }
            MdItemVariant::Chunks { items } => {
                items.push(item.clone());
            }
            MdItemVariant::Item { content } => {
                content.push(item.clone());
            }
        }
    }

    pub fn last_child_mut(&mut self) -> Option<&mut MdItem> {
        match &mut self.variant {
            MdItemVariant::Heading { content, level } => content.last_mut(),
            MdItemVariant::Table { cells } => cells.last_mut().and_then(|l| l.last_mut()),
            MdItemVariant::Text { content } => None,
            MdItemVariant::Chunks { items } => items.last_mut(),
            MdItemVariant::Strong { content } => content.last_mut(),
            MdItemVariant::Emphasis { content } => content.last_mut(),
            MdItemVariant::Item { content } => content.last_mut(),
        }
    }

    pub fn last_child(&self) -> Option<&MdItem> {
        match &self.variant {
            MdItemVariant::Heading { content, level } => content.last(),
            MdItemVariant::Table { cells } => cells.last().and_then(|l| l.last()),
            MdItemVariant::Text { content } => None,
            MdItemVariant::Chunks { items } => items.last(),
            MdItemVariant::Strong { content } => content.last(),
            MdItemVariant::Emphasis { content } => content.last(),
            MdItemVariant::Item { content } => content.last(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MdItemVariant {
    Chunks { items: Vec<MdItem> },
    Text { content: String },
    Heading { content: Vec<MdItem>, level: u16 },
    Table { cells: Vec<Vec<MdItem>> },
    Strong { content: Vec<MdItem> },
    Emphasis { content: Vec<MdItem> },
    Item { content: Vec<MdItem> },
}

#[derive(Clone)]
pub struct MarkdownViewerConfig {
    pub heading_color: Color,
}

impl Default for MarkdownViewerConfig {
    fn default() -> Self {
        Self {
            heading_color: dark_theme_pallete().text,
        }
    }
}

pub struct MarkdownViewer {
    pub(super) original: String,
    pub(super) md_items: Vec<MdItem>,
    pub(super) config: MarkdownViewerConfig,
    pub(super) is_hovered: bool,
}

impl Clone for MarkdownViewer {
    fn clone(&self) -> Self {
        let orig_clone = self.original.clone();
        Self {
            original: orig_clone,
            md_items: self.md_items.clone(),
            is_hovered: self.is_hovered,
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
    pub fn new(original: &str, config: MarkdownViewerConfig) -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        tasks.push(Task::done(Message::Update(original.to_string())));

        (
            Self {
                original: "".to_string(),
                md_items: vec![],
                is_hovered: false,
                config,
            },
            iced::Task::batch(tasks),
        )
    }

    pub fn get_original(&self) -> &str {
        &self.original
    }
}
