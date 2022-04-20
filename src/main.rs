use clap::Parser;
use iced::{window, Application, Settings};

// todo: are these supposed to be included here?
mod application;
mod common_enums;
mod create_goal_page_widget;
mod edit_goal_page_widget;
mod edit_goal_widget;
mod goal_page_widget;
mod goal_widget;
mod goals;
mod tool_bar_widget;

extern crate csv;
extern crate nfd;
// this has to be in the crate root according to Rust's rules
#[macro_use]
extern crate serde_derive;

#[derive(Parser, Debug)]
pub struct ApplicationArgs {
    goals_file: Option<String>,
}

pub fn main() -> iced::Result {
    let args = ApplicationArgs::parse();

    let mut settings = Settings::with_flags(common_enums::ApplicationFlags {
        startup_goals_file_path: match args.goals_file {
            Some(file_path) => {
                let relative_path = std::path::PathBuf::from(file_path);
                let mut absolute_path = std::env::current_dir().unwrap();
                absolute_path.push(relative_path);
                Some(absolute_path.into_os_string().into_string().unwrap())
            }
            None => None,
        },
    });
    settings.window = window::Settings {
        size: (2448, 1024),
        min_size: None,
        max_size: None,
        resizable: true,
        decorations: true,
        transparent: false,
        always_on_top: false,
        icon: None,
    };
    settings.default_font = Some(include_bytes!("C:/Windows/fonts/calibri.ttf"));
    settings.default_text_size = 28;
    application::MyApp::run(settings)
}
