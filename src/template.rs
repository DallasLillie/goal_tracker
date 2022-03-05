// this file is just intended as a template for creating new widgets
use iced::{Column, Command, Element};

use crate::common_enums::Message;

pub struct TemplateWidget {}

impl TemplateWidget {
    pub fn new() -> Self {
        TemplateWidget {}
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            _ => Command::none(),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let mut content = Column::new();

        content.into()
    }
}
