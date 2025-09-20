use framework::Context;
use iced::Subscription;

use super::{NodeAction, TreeNode};

impl TreeNode {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        if let super::tree_node_state::Content::Folder(descriptor) = &self.content {
            for child in &descriptor.children {
                subs.push(child.subscription(ctx));
            }
        }

        subs.push(ctx.focus_manager.unfocus(
            &self.focus_id,
            super::Message::NodeAction(self.id, super::NodeAction::RenameCancel),
        ));

        let id = self.id;
        subs.push(
            ctx.conversations_service
                .update_subscribe_by_id(id, move |dto| {
                    super::Message::NodeAction(id, NodeAction::ActualizedName(dto.name))
                }),
        );

        Subscription::batch(subs)
    }
}
