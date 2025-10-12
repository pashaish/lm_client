use std::{hash::{DefaultHasher, Hash}, sync::{Arc, RwLock}};
use std::hash::Hasher;

use iced::{
    Subscription, Task,
    advanced::{
        self,
        widget::{Id, operate},
    },
    event::listen_raw,
    futures::stream,
    mouse,
};

use crate::migration_14;

#[derive(Debug, Clone)]
pub enum Message {
    RequestFocus,
    FoundFocus(Id),
    ClearFocus,
    StartFindFocus,

    FinalState,
}

#[derive(Debug)]
struct SharedState {
    pub current_focus: Option<Id>,
    pub previous_focus: Option<Id>,
}

#[derive(Clone, Debug)]
pub struct FocusManager {
    state: Arc<RwLock<SharedState>>,
}

impl FocusManager {
    #[must_use] pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(SharedState {
                current_focus: None,
                previous_focus: None,
            })),
        }
    }

    /// # Panics
    pub fn root_update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::RequestFocus => {
                operate(advanced::widget::operation::focusable::count()).map(|count| {
                    match count.focused {
                        Some(_) => Message::StartFindFocus,
                        None => Message::ClearFocus,
                    }
                })
            }
            Message::StartFindFocus => {
                operate(advanced::widget::operation::focusable::find_focused())
                    .map(Message::FoundFocus)
            }
            Message::ClearFocus => {
                let mut state = self.state.write().expect("Failed to write to state");
                state.current_focus = None;

                Self::finalize()
            }
            Message::FoundFocus(new_id) => {
                let mut state = self.state.write().expect("Failed to write to state");
                state.current_focus = Some(new_id);

                Self::finalize()
            }
            Message::FinalState => {
                let mut state = self.state.write().expect("Failed to write to state");
                let current_focus = state.current_focus.clone();
                if state.current_focus.is_some() {
                    state.previous_focus.clone_from(&current_focus);
                }

                Task::none()
            }
        }
    }

    /// # Panics
    #[deprecated]
    #[must_use] pub fn focus_by_raw<T>(&self, raw_id: &'static str) -> iced::Task<T>
    where
        T: Clone + Send + 'static,
    {
        let id = Id::new(&raw_id);
        self.focus(id)
    }

    #[must_use] pub fn focus<T>(&self, id: Id) -> iced::Task<T>
    where
        T: Clone + Send + 'static,
    {
        let mut state = self.state.write().expect("Failed to write to state");
        state.previous_focus = Some(id.clone());
        state.current_focus = Some(id.clone());

        operate(advanced::widget::operation::focusable::focus(id.clone()))
    }

    #[deprecated]
    pub fn unfocus_by_raw<TMessage>(
        &self,
        focus_id: &'static str,
        message: TMessage,
    ) -> iced::Subscription<TMessage>
    where
        TMessage: Clone + Sync + Send + 'static,
    {
        let id = Id::new(&focus_id);
        self.unfocus(id, message)
    }
    
    pub fn unfocus<TMessage>(
        &self,
        focus_id: Id,
        message: TMessage,
    ) -> iced::Subscription<TMessage>
    where
        TMessage: Clone + Sync + Send + 'static,
    {
        if self.was_unfocus(focus_id.clone()) {
            let focus_hash = {
                let mut hasher = DefaultHasher::new();
                focus_id.hash(&mut hasher);
                hasher.finish()
            };

            return migration_14::subscription::run_with_id(
                focus_hash,
                stream::once(async move { message }),
            );
        }

        Subscription::none()
    }

    fn was_changed(&self) -> bool {
        let state = self.state.read().expect("Failed to read from state");
        state.current_focus != state.previous_focus
    }

    fn was_unfocus(&self, id: Id) -> bool {
        let state = self.state.read().expect("Failed to read from state");

        if state.previous_focus != Some(id) {
            return false;
        }

        self.was_changed()
    }

    pub fn root_subscription(&self) -> iced::Subscription<Message> {
        listen_raw(|event, _, _| match event {
            iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                Some(Message::RequestFocus)
            }
            _ => None,
        })
    }

    fn finalize() -> Task<Message> {
        Task::done(Message::FinalState)
    }
}
