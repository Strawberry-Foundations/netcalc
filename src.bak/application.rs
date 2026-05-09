use iced::Size;
use iced::widget::container;
use iced::{Element};

pub struct Application {

}

#[derive(Debug, Clone)]
pub enum Message {
}


impl Default for Application {
    fn default() -> Self {
        Self {

        }
    }
}

impl Application {
    pub fn default_settings() -> iced::Settings {
        iced::Settings {
            antialiasing: true,
            ..Default::default()
        }
    }

    pub fn default_window() -> iced::window::Settings {
        iced::window::Settings {
            size: Size::new(800f32, 600f32),
            resizable: false,
            ..Default::default()
        }
    }

    pub fn update(_state: &mut Application, _message: Message) {}

    pub fn view(_state: &Application) -> Element<'_, Message>  {
        container(iced::widget::Text::new("Hello, world!")).into()
    }
}