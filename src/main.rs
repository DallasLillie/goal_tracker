use clap::Parser;
use iced::{Application, Settings};

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
    #[clap(default_value_t = String::from("..\\resources\\load_test.csv"))]
    goals_file: String,
}

pub fn main() -> iced::Result {
    let args = ApplicationArgs::parse();

    let relative_path = std::path::PathBuf::from(args.goals_file);
    let mut absolute_path = std::env::current_dir().unwrap();
    absolute_path.push(relative_path);

    let settings = Settings::with_flags(common_enums::ApplicationFlags {
        startup_goals_file_path: Some(absolute_path.into_os_string().into_string().unwrap()),
    });
    application::MyApp::run(settings)
}
