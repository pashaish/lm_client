use std::{fmt::Debug, sync::{Arc, RwLock}};

use iced::{Element, Task, widget::text_editor};
use pulldown_cmark::HeadingLevel;
use url::Url;

use crate::{app::common::markdown_viewer::{markdown_viewer_update::ParsingState, markdown_viewer_view::{BASE_TEXT_SIZE, ViewContext}}, overrides};

#[derive(Debug, Clone)]
pub enum Message {
    Update(String),

    StartSelection(usize),
    EndSelection(usize),

    LinkClicked(Url),
}

#[derive(Debug, Clone)]
pub enum MdSpan {
    Text {
        content: String,
        heading_level: Option<HeadingLevel>,
        strong: bool,
        emphasis: bool,
    },
    NewLine,
    TableHeader { columns: usize },
    TableCellPush,
    TableEnd,
}

pub struct MarkdownViewer
{
    pub(super) original: String,
    pub(super) md_spans: Vec<MdSpan>,
}

impl Clone for MarkdownViewer {
    fn clone(&self) -> Self {
        let orig_clone = self.original.clone();
        Self {
            original: orig_clone,
            md_spans: self.md_spans.clone(),
        }
    }
}

impl Debug for MarkdownViewer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownViewer")
            .field("original", &self.original)
            .field("md_spans", &self.md_spans)
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
                md_spans: vec![],
            },
            iced::Task::batch(tasks)
        )
    }

    pub fn get_original(&self) -> &str {
        &self.original
    }
}
