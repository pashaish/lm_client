use iced::Subscription;
use framework::Context;

use super::Rag;

impl Rag {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        subs.push(
            ctx.vector_service
                .subscribe_files_update(self.conversation.id, |_| {
                    super::Message::StartLoadingRagFilesLists
                }),
        );


        subs.push(
            self.model_picker
                .subscription(ctx)
                .map(super::Message::ModelPicker),
        );

        Subscription::batch(subs)
    }
}
