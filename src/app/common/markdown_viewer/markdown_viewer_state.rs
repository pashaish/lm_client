use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

use iced::{Element, Task, widget::text_editor};
use pulldown_cmark::HeadingLevel;
use url::Url;

use crate::{
    app::common::markdown_viewer::{
        markdown_viewer_update::ParsingState,
        markdown_viewer_view::{BASE_TEXT_SIZE, ViewContext},
    },
    overrides,
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
        log::debug!("STR: {str}, self: {self:?}");

        let text_item = MdItem {
            variant: MdItemVarian::Text {
                content: str.to_string(),
            },
            is_completed: true,
        };

        match &mut self.variant {
            MdItemVarian::Heading { content, level } => {
                content.push(text_item);
            }
            MdItemVarian::Table { cells } => {
                panic!("Wrong Insert")
            }
            MdItemVarian::Text { content } => {
                panic!("Wrong Insert")
            }
            MdItemVarian::Chunks { items } => {
                items.push(text_item);
            }
        }
    }

    pub fn last_child_mut(&mut self) -> Option<&mut MdItem> {
        match &mut self.variant {
            MdItemVarian::Heading { content, level } => content.last_mut(),
            MdItemVarian::Table { cells } => cells.last_mut().and_then(|l| l.last_mut()),
            MdItemVarian::Text { content } => None,
            MdItemVarian::Chunks { items } => items.last_mut(),
        }
    }

    pub fn last_child(&self) -> Option<&MdItem> {
        match &self.variant {
            MdItemVarian::Heading { content, level } => content.last(),
            MdItemVarian::Table { cells } => cells.last().and_then(|l| l.last()),
            MdItemVarian::Text { content } => None,
            MdItemVarian::Chunks { items } => items.last(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MdItemVarian {
    Chunks { items: Vec<MdItem> },
    Text { content: String },
    Heading { content: Vec<MdItem>, level: u16 },
    Table { cells: Vec<Vec<MdItem>> },
}

pub struct MarkdownViewer {
    pub(super) original: String,
    pub(super) md_items: Vec<MdItem>,
}

impl Clone for MarkdownViewer {
    fn clone(&self) -> Self {
        let orig_clone = self.original.clone();
        Self {
            original: orig_clone,
            md_items: self.md_items.clone(),
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
    pub fn new(original: &str) -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        tasks.push(Task::done(Message::Update(original.to_string())));

        (
            Self {
                original: "".to_string(),
                md_items: vec![],
            },
            iced::Task::batch(tasks),
        )
    }

    pub fn get_original(&self) -> &str {
        &self.original
    }
}
