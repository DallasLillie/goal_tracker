use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};

mod goals;

extern crate csv;
// this has to be in the crate root according to Rust's rules
#[macro_use]
extern crate serde_derive;

// todo: current goal is to get a window loaded up and a button for loading goals from a file
// it'll then show a preview of those goals
#[derive(Default)]
struct MyApp {
    load_goals_button_state: button::State,
    save_goals_button_state: button::State,
    goals: Vec<goals::Goal>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    LoadGoalsPressed,
    SaveGoalsPressed,
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Goal Tracker")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::LoadGoalsPressed => {
                goals::load_goals(&mut self.goals);
            }
            Message::SaveGoalsPressed => {
                goals::save_goals(&self.goals);
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.load_goals_button_state, Text::new("Load Goals"))
                    .on_press(Message::LoadGoalsPressed),
            )
            .push(
                Button::new(&mut self.save_goals_button_state, Text::new("Save Goals"))
                    .on_press(Message::SaveGoalsPressed),
            )
            .into()
    }
}

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}
