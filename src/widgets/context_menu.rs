use iced::{
    Element, Theme,
    border::Radius,
    widget::{Button, Column, Container, Text},
};

use super::icon::{Icon, IconType};

#[derive(Debug, Clone, Copy)]
pub struct Action<'a, T: Clone> {
    pub(super) icon_color: Option<iced::Color>,
    pub(super) icon: Option<IconType>,
    pub(super) name: &'a str,
    #[allow(clippy::struct_field_names)]
    pub(super) action: T,
}

impl<'a, T: Clone> Action<'a, T> {
    pub const fn new(name: &'a str, action: T) -> Self {
        Self {
            icon_color: None,
            icon: None,
            name,
            action,
        }
    }

    pub const fn icon(mut self, icon: IconType) -> Self {
        self.icon = Some(icon);
        self
    }

    pub const fn icon_color(mut self, color: iced::Color) -> Self {
        self.icon_color = Some(color);
        self
    }
}

pub struct ContextMenu<'a, T>
where
    T: Clone + 'a,
{
    actions: Vec<Action<'a, T>>,
    content: Element<'a, T>,
}

impl<'a, T> ContextMenu<'a, T>
where
    T: Clone + 'a,
{
    pub const fn new(content: Element<'a, T>) -> Self {
        Self {
            content,
            actions: Vec::new(),
        }
    }

    pub fn action(mut self, action: Action<'a, T>) -> Self {
        self.actions.push(action);
        self
    }

    fn view(self) -> iced::Element<'a, T> {
        let actions = self.actions.clone();
        iced_aw::ContextMenu::new(self.content, move || {
            let mut column = Column::new();
            for action in actions.clone() {
                let mut row = iced::widget::Row::new()
                    .spacing(10)
                    .align_y(iced::alignment::Alignment::Center);

                if let Some(icon) = action.icon {
                    row = row.push(Container::new(Icon::new(icon)).style(move |theme: &Theme| {
                        let palette = theme.extended_palette();

                        let color = action.icon_color.map_or(palette.secondary.base.text, |color| color);

                        iced::widget::container::Style {
                            text_color: Some(color),
                            ..Default::default()
                        }
                    }));
                }

                row = row.push(
                    Text::new(action.name)
                        .size(12)
                        .width(iced::Length::FillPortion(1))
                        .align_x(iced::alignment::Alignment::Center),
                );

                column = column.push(
                    Button::new(row)
                        .width(120)
                        .style(|theme: &Theme, status| {
                            let mut style = iced::widget::button::secondary(theme, status);
                            style.border.radius = Radius::new(0.0);
                            style.background = Some(
                                style
                                    .background
                                    .expect("background not set")
                                    .scale_alpha(0.95),
                            );
                            style
                        })
                        .on_press(action.action.clone()),
                );
            }

            column.into()
        })
        .into()
    }
}

impl<'a, T> From<ContextMenu<'a, T>> for iced::Element<'a, T>
where
    T: Clone + 'a,
{
    fn from(context_menu: ContextMenu<'a, T>) -> Self {
        context_menu.view()
    }
}
