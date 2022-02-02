use iced::{
    button, Align, Button, Column, Container, Element, Row, Sandbox, Scrollable, Settings, Text,
};

mod goals;

extern crate csv;
// this has to be in the crate root according to Rust's rules
#[macro_use]
extern crate serde_derive;

// todo: current goal is to get a window loaded up and a button for loading goals from a file
// it'll then show a preview of those goals
struct MyApp {
    load_goals_button_state: button::State,
    save_goals_button_state: button::State,
    goals: Vec<goals::Goal>,
    goal_page: GoalPageWidget,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    LoadGoalsPressed,
    SaveGoalsPressed,
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        MyApp {
            load_goals_button_state: button::State::new(),
            save_goals_button_state: button::State::new(),
            goals: Vec::new(),
            goal_page: GoalPageWidget::new(),
        }
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
        let content = Row::new()
            .push(
                Column::new()
                    .align_items(Align::Center)
                    .push(
                        Button::new(&mut self.load_goals_button_state, Text::new("Load Goals"))
                            .on_press(Message::LoadGoalsPressed),
                    )
                    .push(
                        Button::new(&mut self.save_goals_button_state, Text::new("Save Goals"))
                            .on_press(Message::SaveGoalsPressed),
                    ),
            )
            .push(self.goal_page.view());
        // let scrollable = Scrollable::new(&mut self.scroo).push(Container::new(content));
        Container::new(content).into()
    }
}

struct GoalPageWidget {
    goal_entries: Vec<GoalWidget>,
}

impl GoalPageWidget {
    fn new() -> Self {
        GoalPageWidget {
            goal_entries: vec![GoalWidget::new(), GoalWidget::new(), GoalWidget::new()],
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut content = Column::new();

        for goal_entry in self.goal_entries.iter_mut() {
            content = content.push(goal_entry.view());
        }

        content.into()
    }
}

struct GoalWidget {
    text: String,
}

impl GoalWidget {
    fn new() -> Self {
        GoalWidget {
            text: "Daily Goal #".to_owned(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        Row::new().push(Text::new(self.text.to_string())).into()
    }
}

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}
