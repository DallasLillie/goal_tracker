use iced::{Element, Sandbox, Settings, Text};

mod goals;

extern crate csv;
// this has to be in the crate root according to Rust's rules
#[macro_use]
extern crate serde_derive;

// todo: current goal is to get a window loaded up and a button for loading goals from a file
// it'll then show a preview of those goals
struct MyApp;

impl Sandbox for MyApp {
    type Message = ();

    fn new() -> MyApp {
        MyApp
    }

    fn title(&self) -> String {
        String::from("Goal Tracker")
    }

    fn update(&mut self, _message: Self::Message) {
        // This app has no interactions
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello World").into()
    }
}

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}
