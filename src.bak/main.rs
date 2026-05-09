use crate::application::Application;
use iced::theme::Theme;

pub mod application;

fn main() -> iced::Result {
    iced::application(Application::default, Application::update, Application::view)
        .settings(Application::default_settings())
        .theme(Theme::Ferra)
        .title("NetCalc")
        .window(Application::default_window())
        .run()
}