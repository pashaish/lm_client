use app::App;
use framework::utils::APP_NAME;
use iced::Theme;
use theme::dark_theme::dark_theme;

mod app;
mod theme;
mod widgets;

// #[tokio::main]
fn main() {
    env_logger::init();

    iced::application::application(
        App::new,
        App::update,
        App::view,
    )
    .subscription(App::subscription)
    // .theme(|_| Theme::Custom(dark_theme()))
    .title(APP_NAME)
    .run()
    .expect("Failed to run the application");
}
