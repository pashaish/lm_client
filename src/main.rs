use app::App;
use framework::utils::APP_NAME;
use iced::Theme;
use theme::dark_theme::dark_theme;

mod app;
mod theme;
mod widgets;
mod overrides;

#[tokio::main]
async fn main() {
    env_logger::init();

    iced::application(APP_NAME, App::update, App::view)
        .theme(|_| Theme::Custom(dark_theme()))
        .subscription(App::subscription)
        .run_with(App::new)
        .expect("Failed to run the application");
}
