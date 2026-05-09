#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use crate::application::Application;
use iced::theme::Theme;

mod application;
mod fonts;
mod theme;

fn main() -> iced::Result {
    iced::application(Application::default, Application::update, Application::view)
        .settings(Application::default_settings())
        .theme(Theme::Ferra)
        .title("Network Calculator 1.0.0")
        .window(Application::default_window())
        .run()
}