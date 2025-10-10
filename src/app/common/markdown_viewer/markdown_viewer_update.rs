use crate::app::common::markdown_viewer::{markdown_viewer_state::MdSpan, markdown_viewer_view::BASE_TEXT_SIZE};

use super::MarkdownViewer;
use iced::Task;
use pulldown_cmark::{CowStr, HeadingLevel};

pub(super) struct ParsingState {
    pub(super) in_paragraph: bool,
    pub(super) heading_level: Option<HeadingLevel>,
    pub(super) strong: bool,
    pub(super) emphasis: bool,
}

impl MarkdownViewer {
    pub fn update(&mut self, message: super::Message) -> Task<super::Message> {
        match message {

            super::Message::Update(original) => {
                self.original = original;

                let parser = pulldown_cmark::Parser::new_ext(
                    self.original.clone().leak(),
                    pulldown_cmark::Options::all(),
                );
                self.md_spans.clear();

                let mut parsing_state = ParsingState {
                    in_paragraph: false,
                    heading_level: None,
                    emphasis: false,
                    strong: false,
                }; 
                for event in parser {
                    if !self.handle_md_event(&event, &mut parsing_state) {
                        break;
                    }
                }

                Task::none()
            }
            super::Message::StartSelection(id) => {
                log::debug!("StartSelection: {}", id);
                Task::none()
            }
            super::Message::EndSelection(_id) => {
                log::debug!("EndSelection: {}", _id);
                Task::none()
            }
            super::Message::LinkClicked(url) => {
                log::info!("Link clicked: {}", url);
                Task::none()
            }
        }
    }

    fn handle_md_event(&mut self, event: &pulldown_cmark::Event, parsing_state: &mut ParsingState) -> bool {
        match event {
            pulldown_cmark::Event::Start(tag) => match tag {
                pulldown_cmark::Tag::Paragraph => parsing_state.in_paragraph = true,
                pulldown_cmark::Tag::Strong => parsing_state.strong = true,
                pulldown_cmark::Tag::Emphasis => parsing_state.emphasis = true,
                pulldown_cmark::Tag::Heading { level, .. } => {
                    parsing_state.heading_level = Some(*level);
                }
                pulldown_cmark::Tag::Table(aligments) => {
                    self.md_spans.push(MdSpan::NewLine);
                }
                _ => {}
            },
            pulldown_cmark::Event::Text(text) => {
                self.add_text(text.clone(), parsing_state);
            }
            pulldown_cmark::Event::End(tag) => match tag {
                pulldown_cmark::TagEnd::Paragraph => {
                    parsing_state.in_paragraph = false;
                    self.md_spans.push(MdSpan::NewLine);
                }
                pulldown_cmark::TagEnd::Heading { .. } => {
                    parsing_state.heading_level = None;
                    self.md_spans.push(MdSpan::NewLine);
                }
                pulldown_cmark::TagEnd::BlockQuote(_) |
                pulldown_cmark::TagEnd::Item |
                pulldown_cmark::TagEnd::Link { .. } |
                pulldown_cmark::TagEnd::CodeBlock => {
                    self.md_spans.push(MdSpan::NewLine);
                } 
                pulldown_cmark::TagEnd::Emphasis => {
                    parsing_state.emphasis = false;
                }
                pulldown_cmark::TagEnd::Strong => {
                    parsing_state.strong = false;
                }
                _ => {}
            },
            _ => {}
        }

        true
    }

    fn add_text(&mut self, text: CowStr, parsing_state: &ParsingState) {
        for line in text.lines() {
            self.md_spans.push(MdSpan::Text {
                content: line.to_string(),
                heading_level: parsing_state.heading_level,
                strong: parsing_state.strong,
                emphasis: parsing_state.emphasis,
            });
        }
    }

    pub fn set_original(&mut self, new_original: String) -> Task<super::Message> {
        self.update(super::Message::Update(new_original))
    }

    pub fn append(&mut self, to_append: &str) -> Task<super::Message> {
        let mut new_original = self.original.clone();
        new_original.push_str(to_append);
        self.update(super::Message::Update(new_original))
    }
}
