use crate::{
    app::common::markdown_viewer::{
        markdown_viewer_state::{MdItem, MdItemVarian},
        markdown_viewer_view::BASE_TEXT_SIZE,
    },
    overrides::table,
};

use super::MarkdownViewer;
use iced::Task;
use pulldown_cmark::{CowStr, HeadingLevel};

pub(super) struct ParsingState {
    pub(super) in_paragraph: bool,
    pub(super) heading_level: Option<HeadingLevel>,
    pub(super) strong: bool,
    pub(super) emphasis: bool,

    pub(super) in_table: bool,
    pub(super) table: Vec<Vec<MdItem>>,
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
                self.md_items.clear();

                let mut parsing_state = ParsingState {
                    in_paragraph: false,
                    heading_level: None,
                    emphasis: false,
                    strong: false,
                    in_table: false,
                    table: Vec::new(),
                };

                let mut new_md_items = Vec::new();

                for event in parser {
                    if !self.handle_md_event(&event, &mut parsing_state, &mut new_md_items) {
                        break;
                    }
                }

                self.md_items = new_md_items;

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

    fn handle_md_event(
        &mut self,
        event: &pulldown_cmark::Event,
        parsing_state: &mut ParsingState,
        container: &mut Vec<MdItem>,
    ) -> bool {
        match event {
            pulldown_cmark::Event::Start(tag) => match tag {
                pulldown_cmark::Tag::Heading { level, .. } => {
                    container.push(MdItem {
                        is_completed: false,
                        variant: MdItemVarian::Heading {
                            level: Self::heading_level_to_u16(level),
                            content: vec![],
                        },
                    });
                }
                pulldown_cmark::Tag::Table(_table) => {
                    parsing_state.in_table = true;
                }
                pulldown_cmark::Tag::TableRow => {
                    parsing_state.table.push(vec![]);
                }
                pulldown_cmark::Tag::TableCell => {
                    parsing_state.table.last_mut().unwrap().push(MdItem {
                        variant: MdItemVarian::Chunks { items: vec![] },
                        is_completed: false,
                    });
                }
                pulldown_cmark::Tag::TableHead => {
                    parsing_state.table.push(vec![]);
                }
                _ => {}
            },
            pulldown_cmark::Event::Text(text) => {
                let Some(last) = container.last_mut() else {
                    log::warn!("Last not found");
                    return true;
                };

                let text = text.parse::<String>().unwrap();

                if parsing_state.in_table {
                    if let Some(item) = parsing_state.table.last_mut().and_then(|l| l.last_mut()) {
                        let uncompleted_item = Self::find_last(item);
                        uncompleted_item.push_text(&text);
                    }
                } else {
                    let uncompleted_item = Self::find_last(last);
                    uncompleted_item.push_text(&text);
                }
            }
            pulldown_cmark::Event::End(tag) => match tag {
                pulldown_cmark::TagEnd::Table => {
                    parsing_state.in_table = false;

                    container.push(MdItem {
                        variant: MdItemVarian::Table { cells: parsing_state.table.clone() },
                        is_completed: false,
                    });

                    parsing_state.table.clear();
                }
                _ => {}
            },
            _ => {}
        }

        true
    }

    fn find_last<'a>(container: &'a mut MdItem) -> &'a mut MdItem {
        if container.is_completed == false {
            if let Some(child) = container.last_child() {
                if child.is_completed == true {
                    return container;
                }

                return Self::find_last(container.last_child_mut().unwrap());
            } else {
                return container;
            }
        }

        panic!("Something do wrong");
    }

    pub fn heading_level_to_u16(level: &HeadingLevel) -> u16 {
        match level {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
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
