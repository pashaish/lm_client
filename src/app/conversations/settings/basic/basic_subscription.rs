use framework::{utils::event_system::Event, Context};
use iced::Subscription;

use super::Basic;

const PRESETS_UPDATED_EVENT: &Event = &Event::UpdatePresets(vec![]);

impl Basic {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        subs.push(
            ctx.conversations_service
                .update_subscribe(&self.conversation, |dto| super::Message::RenameComplete(dto.name)),
        );
        subs.push(
            ctx.event_system
                .subscribe(PRESETS_UPDATED_EVENT, super::Message::PresetsLoaded),
        );

        subs.push(self.model_picker.subscription(ctx).map(super::Message::ModelPicker));

        Subscription::batch(subs)
    }
}
