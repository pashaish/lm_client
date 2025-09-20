use framework::{
    utils::{
        focus_manager, take_component
    }, Context
};

use super::{
    conversations::{self, Conversations},
    presets::{self, Presets},
    settings::{self, Settings},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    Conversations,
    Presets,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    Conversations(conversations::Message),
    Presets(presets::Message),
    Settings(settings::Message),
    StartChangeView(View),
    CompleteChangeView(Option<View>),

    FocusManager(focus_manager::Message),
}

pub struct App {
    // Components
    pub(super) conversations: Conversations,
    pub(super) presets: Presets,
    pub(super) settings: Settings,

    // State
    pub(super) current_view: View,

    // Context
    pub(super) context: Context,
}

impl App {
    pub fn new() -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        let context = framework::Context::new();

        (
            Self {
                context,
                current_view: View::Conversations,
                conversations: take_component(
                    &mut tasks,
                    Message::Conversations,
                    Conversations::new(),
                ),
                presets: take_component(&mut tasks, Message::Presets, Presets::new()),
                settings: take_component(&mut tasks, Message::Settings, Settings::new()),
            },
            iced::Task::batch(tasks),
        )
    }
}
