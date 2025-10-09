use std::{fmt::Debug, sync::{Arc, RwLock}};

use iced::{Element, Task, widget::text_editor};

use crate::app::common::markdown_viewer::markdown_viewer_view::{BASE_TEXT_SIZE, ViewContext};

#[derive(Debug, Clone)]
pub enum Message {
    Update(String),
    
    Nothing,
}

#[derive(Debug, Clone)]
pub(super) enum MdNode {
    Root { children: Vec<MdNode> },
    Paragraph { children: Vec<MdNode> },
    Text { value: String },
    Heading { level: u8, children: Vec<MdNode> },
}

pub struct MarkdownViewer {
    pub(super) original: String,
    pub(super) node: MdNode,
    pub(super) view_cache: Option<Element<'static, Message>>,
}

impl Clone for MarkdownViewer {
    fn clone(&self) -> Self {
        Self {
            original: self.original.clone(),
            node: self.node.clone(),
            view_cache: None,
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
                view_cache: None,
            },
            iced::Task::batch(tasks)
        )
    }

    pub fn get_original(&self) -> &str {
        &self.original
    }

    pub(super) fn mdast_to_node(mdast: &markdown::mdast::Node) -> MdNode {
        match mdast {
            markdown::mdast::Node::Root(root) => MdNode::Root { children: root.children.iter().map(Self::mdast_to_node).collect() },
            markdown::mdast::Node::Paragraph(paragraph) => MdNode::Paragraph { children: paragraph.children.iter().map(Self::mdast_to_node).collect() },
            markdown::mdast::Node::Text(text) => MdNode::Text {
                value: text.value.clone(),
            },
            markdown::mdast::Node::Heading(heading) => MdNode::Heading { level: heading.depth, children: heading.children.iter().map(Self::mdast_to_node).collect() },
            _ => MdNode::Text {
                value: "[Unsupported node]".into(),
            },
        }
    }
}
