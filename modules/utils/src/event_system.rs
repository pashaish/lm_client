use std::{
    any::Any, fmt::Debug, hash::Hash, sync::{Arc, RwLock}
};

use iced::{Subscription, futures::SinkExt, stream};
use types::{common::ProgressStatus, dto::{ConversationNodeDTO, ConversationNodeID, MessageDTO, MessageID, PresetDTO, ProviderDTO, RagFileDTO}};

#[derive(Debug, Clone)]
pub enum Event {
    ConversationUpdate(ConversationNodeDTO),
    ConversationReceiveMessage(MessageDTO),
    MessageDelete(MessageID),
    RagFilesUpdated {
        conversation_id: ConversationNodeID,
        files: Vec<RagFileDTO>,
    },
    ConversationDelete(ConversationNodeID),
    ProvidersUpdate(Vec<ProviderDTO>),
    LoadingFilesStatus(ProgressStatus),
    UpdatePresets(Vec<PresetDTO>),
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ConversationUpdate(a), Self::ConversationUpdate(b)) => a.id == b.id,

            (Self::MessageDelete(a), Self::MessageDelete(b)) |
            (Self::ConversationDelete(a), Self::ConversationDelete(b)) |
            (
                Self::RagFilesUpdated {
                    conversation_id: a, ..
                },
                Self::RagFilesUpdated {
                    conversation_id: b, ..
                },
            ) => a == b,

            (Self::ConversationReceiveMessage(a), Self::ConversationReceiveMessage(b)) => {
                a.conversation_id == b.conversation_id
            }
            (Self::LoadingFilesStatus(_a), Self::LoadingFilesStatus(_b)) => true,
            (Self::ProvidersUpdate(_a), Self::ProvidersUpdate(_b)) => true,
            (Self::UpdatePresets(_a), Self::UpdatePresets(_b)) => true,
            _ => false,
        }
    }
}

impl Event {
    #[must_use] pub fn get_data(&self) -> Box<dyn Any> {
        match self {
            Self::UpdatePresets(data) => Box::new(data.clone()),
            Self::LoadingFilesStatus(data) => Box::new(data.clone()),
            Self::ProvidersUpdate(data) => Box::new(data.clone()),
            Self::ConversationReceiveMessage(data) => Box::new(data.clone()),
            
            Self::ConversationUpdate(data) => Box::new(data.clone()),
            
            Self::MessageDelete(data) |
            Self::ConversationDelete(data) => Box::new(*data),

            Self::RagFilesUpdated {
                conversation_id,
                files,
            } => Box::new((*conversation_id, files.clone())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventSystem {
    queue: Arc<RwLock<Vec<Event>>>,
    events: Arc<RwLock<Vec<Event>>>,
}

impl EventSystem {
    #[must_use] pub fn new() -> Self {
        Self {
            queue: Arc::new(RwLock::new(vec![])),
            events: Arc::new(RwLock::new(vec![])),
        }
    }

    /// # Panics
    pub fn pre_update(&mut self) {
        let mut events_queue = self.queue.write().expect("Failed to write to queue");
        let mut events = self.events.write().expect("Failed to write to events");

        for event in events_queue.iter() {
            events.push(event.clone());
        }

        events_queue.clear();
    }

    /// # Panics
    pub fn post_subscribe(&self) {
        let mut events = self.events.write().expect("Failed to write to events");

        events.clear();
    }

    /// # Panics
    pub fn dispatch(&mut self, event: Event) {
        let mut queue = self.queue.write().expect("Failed to write to queue");

        queue.push(event);
    }

    /// # Panics
    pub fn subscribe<TData, TMessage>(
        &self,
        event: &Event,
        converter: impl Fn(TData) -> TMessage + 'static,
    ) -> iced::Subscription<TMessage>
    where
        TMessage: Debug + Send + Hash + Clone + 'static,
        TData: Clone + Send + 'static,
    {
        let mut subs = vec![];
        let events = self.events.read().expect("Failed to read events");

        for c_event in events.iter() {
            if *event == *c_event {
                let data = c_event.get_data();

                let data = data.downcast::<TData>().expect("Failed to downcast");
                let data = data.as_ref().clone();
                let data = converter(data);

                // ?TODO: NEED UPDATE
                // subs.push(Subscription::run(
                //     Uuid::new_v4().to_string(),
                //     stream::once(async move { data }),
                // ));
                let sub = Subscription::run_with(
                    data,
                    // |data| stream::once(data)
                    |data| {
                        let data = data.clone();
                        stream::channel(1, async |mut out| { out.send(data).await.unwrap(); })
                    }
                );

                subs.push(sub);
            }
        }

        Subscription::batch(subs)
    }
}
