                // TODO: NEED UPDATE

// use iced::{Element, Theme, advanced::graphics::core::Element};

// #[derive(Debug, Clone, Copy)]
// pub enum IconName {
//     Comments,
//     Folder,
//     FolderOpen,
//     FolderPlus,
//     Plus,
//     CirclePlus,
//     SquarePlus,
//     Pencil,
//     FloppyDisk,
//     XMark,
//     Trash,
//     Message,
//     Gear,
//     Box,
//     GripVertical,
//     Spinner,
//     ChevronDown,
//     ChevronRight,
//     ChevronLeft,
//     PaperClip,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum IconType {
//     Solid(IconName),
//     Regular(IconName),
//     #[allow(dead_code)]
//     Brands(IconName),
// }

// const fn icon_name_to_str<'a>(icon_name: IconName) -> &'a str {
//     match icon_name {
//         IconName::Comments => "comments",
//         IconName::Folder => "folder",
//         IconName::FolderOpen => "folder-open",
//         IconName::FolderPlus => "folder-plus",
//         IconName::Plus => "plus",
//         IconName::CirclePlus => "circle-plus",
//         IconName::SquarePlus => "square-plus",
//         IconName::Pencil => "pencil",
//         IconName::FloppyDisk => "floppy-disk",
//         IconName::XMark => "xmark",
//         IconName::Trash => "trash",
//         IconName::Message => "message",
//         IconName::Gear => "gear",
//         IconName::Box => "box",
//         IconName::GripVertical => "grip-vertical",
//         IconName::Spinner => "spinner",
//         IconName::ChevronDown => "chevron-down",
//         IconName::ChevronRight => "chevron-right",
//         IconName::PaperClip => "paperclip",
//         IconName::ChevronLeft => "chevron-left",
//     }
// }

// pub struct Icon {
//     icon: IconType,
//     alpha: f32,
// }

// impl Icon {
//     pub const fn new(icon: IconType) -> Self {
//         Self { icon, alpha: 1.0 }
//     }

//     pub const fn alpha(mut self, alpha: f32) -> Self {
//         self.alpha = alpha;
//         self
//     }

//     pub fn view(self) -> iced::Element<'_> {
//         icon_element(self.icon).style(move |theme: &Theme| {
//             let palette = theme.extended_palette();
//             iced::widget::text::Style {
//                 color: Some(palette.secondary.base.text.scale_alpha(self.alpha)),
//             }
//         })
//     }
// }

// // fn icon_element<'a>(icon_type: IconType) -> iced_font_awesome::FaIcon<'a, Theme> {
// //     match icon_type {
// //         IconType::Solid(icon_name) => iced_font_awesome::fa_icon_solid(icon_name_to_str(icon_name)),
// //         IconType::Regular(icon_name) => iced_font_awesome::fa_icon(icon_name_to_str(icon_name)),
// //         IconType::Brands(icon_name) => {
// //             iced_font_awesome::fa_icon_brands(icon_name_to_str(icon_name))
// //         }
// //     }
// // }

// // impl From<Icon> for iced_font_awesome::FaIcon<'static, Theme> {
// //     fn from(icon: Icon) -> Self {
// //         icon.view()
// //     }
// // }

// // impl<T> From<Icon> for Element<'_, T> {
// //     fn from(icon: Icon) -> Self {
// //         icon.view().into()
// //     }
// // }
