use std::{fmt::Debug, sync::{Arc, RwLock}};

use iced::{Element, Task, widget::text_editor};

use crate::app::common::markdown_viewer::markdown_viewer_view::{BASE_TEXT_SIZE, ViewContext};

#[derive(Debug, Clone, Hash)]
pub enum Message {
    Update(String),

    StartSelection(usize),
    TempEndSelection(usize),
    EndSelection,
    
    // Nothing,
}

#[derive(Debug, Clone, Default)]
pub(super) struct Char {
    pub value: char,
    pub id: usize,
}

#[derive(Debug, Clone)]
pub(super) enum MdNode {
    Root { children: Vec<MdNode> },
    Paragraph { children: Vec<MdNode> },
    Text { value: Vec<Char> },
    Heading { level: u8, children: Vec<MdNode> },
}

pub struct MarkdownViewer {
    pub(super) original: String,
    pub(super) node: MdNode,

    pub(super) start_selection: Option<usize>,
    pub(super) end_selection: Option<usize>,
    pub(super) temp_end_selection: Option<usize>,
}

impl Clone for MarkdownViewer {
    fn clone(&self) -> Self {
        Self {
            original: self.original.clone(),
            node: self.node.clone(),

            start_selection: self.start_selection,
            end_selection: self.end_selection,
            temp_end_selection: self.temp_end_selection,
        }
    }
}

impl Debug for MarkdownViewer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownViewer")
            .field("original", &self.original)
            .field("node", &self.node)
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
                node: MdNode::Root { children: Vec::new() },

                start_selection: None,
                end_selection: None,
                temp_end_selection: None,
            },
            iced::Task::batch(tasks)
        )
    }

    pub fn get_original(&self) -> &str {
        &self.original
    }

    pub(super) fn mdast_to_node(mdast: &markdown::mdast::Node, last_id: &mut usize) -> MdNode {
        match mdast {
            markdown::mdast::Node::Root(root) => MdNode::Root { children: root.children.iter().map(|child| Self::mdast_to_node(child, last_id)).collect() },
            markdown::mdast::Node::Paragraph(paragraph) => MdNode::Paragraph { children: paragraph.children.iter().map(|child| Self::mdast_to_node(child, last_id)).collect() },
            markdown::mdast::Node::Text(text) => MdNode::Text {
                value: text.value.chars().map(|c| {
                    *last_id += 1;
                    Char {
                        value: c,
                        id: *last_id,
                    }
                }).collect(),
            },
            markdown::mdast::Node::Heading(heading) => MdNode::Heading { level: heading.depth, children: heading.children.iter().map(|child| Self::mdast_to_node(child, last_id)).collect() },
            _ => MdNode::Text {
                value: Default::default(),
            },
        }
    }

    pub(super) fn id_is_selected(&self, id: usize) -> bool {
        if let (Some(start), Some(end)) = (self.start_selection, self.temp_end_selection) {
            if start > end {
                return id >= end && id <= start;
            } else {
                return id >= start && id <= end;
            }
        }

        false
    }
}
