use crate::app::common::markdown_viewer::markdown_viewer_state::MdNode;

use super::MarkdownViewer;
use iced::Task;

impl MarkdownViewer {
    pub fn update(&mut self, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::Update(original) => {
                self.original = original;

                if let Ok(node) = &markdown::to_mdast(&self.original, &markdown::ParseOptions::default()) {
                    self.node = Self::mdast_to_node(node, &mut 0);
                } else {
                    self.node = MdNode::Root {
                        children: Vec::new(),
                    };
                }

                Task::none()
            }
            super::Message::StartSelection(id) => {
                log::debug!("StartSelection: {}", id);
                self.start_selection = Some(id);
                self.end_selection = None;
                self.temp_end_selection = None;
                Task::none()
            },
            super::Message::EndSelection => {
                log::debug!("EndSelection");

                self.end_selection = self.temp_end_selection;

                Task::none()
            },
            super::Message::TempEndSelection(id) => {
                log::debug!("TempEndSelection: {}", id);

                self.temp_end_selection = Some(id);

                Task::none()
            },
            // super::Message::Nothing => Task::none(),
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
