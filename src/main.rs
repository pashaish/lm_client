use app::App;
use framework::utils::APP_NAME;
use iced::Theme;
use theme::dark_theme::dark_theme;

mod app;
mod theme;
mod widgets;
mod overrides;

// #[tokio::main]
fn main() {
    env_logger::init();

    iced::application(App::new, App::update, App::view)
        // TODO: NEED RETURN THEME
        // .theme(|_| Theme::Custom(dark_theme()))
        .subscription(App::subscription)
        .run()
        .expect("Failed to run the application");
}
