use iced::Color;
use iced::theme::{Custom, Palette};
use std::sync::Arc;

pub fn dark_theme() -> Arc<Custom> {
    Arc::new(Custom::new(
        "Dark".to_string(),
        Palette {
            background: Color::parse("#1E1E1E").unwrap(),
            text: Color::parse("#D4D4D4").unwrap(),
            primary: Color::parse("#b86725").unwrap(),
            success: Color::parse("#4EC9B0").unwrap(),
            danger: Color::parse("#FF5555").unwrap(),
            warning: Color::parse("#FFAA00").unwrap(),
        },
    ))
}
