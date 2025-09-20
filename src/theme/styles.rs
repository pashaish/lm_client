use iced::{widget::scrollable, Shadow, Theme};

pub enum Side {
    Left,
    Right,
    #[allow(dead_code)]
    Top,
    #[allow(dead_code)]
    Bottom,
}

pub fn fake_oneside_border(theme: &Theme, side: &Side) -> Shadow {
    let palette = theme.extended_palette();

    iced::Shadow {
        color: palette.background.strong.color,
        offset: define_size_vector(side) * 1.0,
        blur_radius: 0.0,
    }
}

#[allow(dead_code)]
pub fn fake_oneside_border_primary(theme: &Theme, side: &Side) -> Shadow {
    let palette = theme.extended_palette();

    iced::Shadow {
        color: palette.primary.strong.color,
        offset: define_size_vector(side) * 2.0,
        blur_radius: 0.0,
    }
}

const fn define_size_vector(side: &Side) -> iced::Vector {
    match side {
        Side::Left => iced::Vector::new(-1.0, 0.0),
        Side::Right => iced::Vector::new(1.0, 0.0),
        Side::Top => iced::Vector::new(0.0, -1.0),
        Side::Bottom => iced::Vector::new(0.0, 1.0),
    }
}

pub fn scrollable_style(theme: &Theme, status: scrollable::Status) -> iced::widget::scrollable::Style {
    let mut style = scrollable::default(theme, status);

    style.vertical_rail.scroller.color = style.vertical_rail.scroller.color.scale_alpha(0.15);
    style.container.background = None;
    style.vertical_rail.background = None;

    style
}