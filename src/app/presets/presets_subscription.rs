use framework::Context;
use iced::{event::listen_raw, keyboard::{self, key::{Code, Physical}}, Event, Subscription};

use super::Presets;

impl Presets {
    pub fn subscription(&self, _ctx: &Context) -> Subscription<super::Message> {
        let subs = vec![];

        Subscription::batch(subs)
    }

    pub fn selected_subscription(&self, _ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        subs.push(
            listen_raw(|event, _, _| {
                match event {
                    Event::Keyboard(keyboard::Event::KeyPressed { key: _, modified_key: _, physical_key, location: _, modifiers, text: _ }) => {
                        match physical_key {
                            Physical::Code(Code::Delete) if modifiers.command() => {
                                Some(super::Message::DeletePreset)
                            },
                            Physical::Code(Code::Backspace) if modifiers.macos_command() => {
                                Some(super::Message::DeletePreset)
                            },
                            Physical::Code(Code::KeyS) if modifiers.command() => {
                                Some(super::Message::CommitChanges)
                            }
                            _ => None,
                        }
                    },
                    _ => None,
                }
            }),
        );

        Subscription::batch(subs)
    }
}
