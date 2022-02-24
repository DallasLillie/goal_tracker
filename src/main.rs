use iced::{Application, Settings};

// todo: are these supposed to be included here?
mod application;
mod common_enums;
mod create_goal_page_widget;
mod goal_page_widget;
mod goal_widget;
mod goals;
mod tool_bar_widget;

extern crate csv;
// this has to be in the crate root according to Rust's rules
#[macro_use]
extern crate serde_derive;

pub fn main() -> iced::Result {
    application::MyApp::run(Settings::default())
}
