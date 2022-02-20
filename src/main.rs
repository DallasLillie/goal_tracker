use iced::{
    button, Align, Button, Color, Column, Container, Element, Font, Length, Row, Sandbox,
    Scrollable, Settings, Text,
};

mod goals;

extern crate csv;
// this has to be in the crate root according to Rust's rules
#[macro_use]
extern crate serde_derive;

struct MyApp {
    goals: Vec<goals::Goal>,
    goal_page: GoalPageWidget,
    tool_bar: ToolBarWidget,
    current_page: ApplicationPage,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    LoadGoalsPressed,
    SaveGoalsPressed,
    CreateGoalPressed,
    YearlyReviewPressed,
    MonthlyReviewPressed,
    WeeklyReviewPressed,
}

enum ApplicationPage {
    HomePage,
    CreateGoalPage,
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        MyApp {
            goals: Vec::new(),
            goal_page: GoalPageWidget::new(),
            tool_bar: ToolBarWidget::new(),
            current_page: ApplicationPage::HomePage,
        }
    }

    fn title(&self) -> String {
        String::from("Goal Tracker")
    }

    fn update(&mut self, message: Self::Message) {
        match self.current_page {
            ApplicationPage::HomePage => match message {
                Message::CreateGoalPressed => {
                    // todo: This should probably be a Message::ChangePage(ApplicationPage) sent by the tool bar actually
                    self.current_page = ApplicationPage::CreateGoalPage;
                }
                _ => {
                    self.goal_page.update(message);
                }
            },
            ApplicationPage::CreateGoalPage => {} // todo: what is it to create a goal?
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        match self.current_page {
            ApplicationPage::HomePage => {
                let content = Column::new()
                    .push(Text::new("Home"))
                    .push(self.tool_bar.view())
                    .push(self.goal_page.view());
                // let scrollable = Scrollable::new(&mut self.scroo).push(Container::new(content));
                Container::new(content).into()
            }
            ApplicationPage::CreateGoalPage => {
                let content = Column::new().push(Text::new("Create New Goal"));
                Container::new(content).into()
            }
        }
    }
}

struct ToolBarWidget {
    load_goals_button_state: button::State,
    save_goals_button_state: button::State,
    create_goal_button_state: button::State,
    yearly_review_button_state: button::State,
    monthly_review_button_state: button::State,
    weekly_review_button_state: button::State,
}

impl ToolBarWidget {
    fn new() -> Self {
        ToolBarWidget {
            load_goals_button_state: button::State::new(),
            save_goals_button_state: button::State::new(),
            create_goal_button_state: button::State::new(),
            yearly_review_button_state: button::State::new(),
            monthly_review_button_state: button::State::new(),
            weekly_review_button_state: button::State::new(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        Row::new()
            .push(
                Button::new(&mut self.load_goals_button_state, Text::new("Load Goals"))
                    .on_press(Message::LoadGoalsPressed),
            )
            .push(
                Button::new(&mut self.save_goals_button_state, Text::new("Save Goals"))
                    .on_press(Message::SaveGoalsPressed),
            )
            .push(
                Button::new(
                    &mut self.create_goal_button_state,
                    Text::new("Create New Goal"),
                )
                .on_press(Message::CreateGoalPressed),
            )
            .push(
                Button::new(
                    &mut self.yearly_review_button_state,
                    Text::new("Yearly Review"),
                )
                .on_press(Message::YearlyReviewPressed),
            )
            .push(
                Button::new(
                    &mut self.monthly_review_button_state,
                    Text::new("Monthly Review"),
                )
                .on_press(Message::MonthlyReviewPressed),
            )
            .push(
                Button::new(
                    &mut self.weekly_review_button_state,
                    Text::new("Weekly Review"),
                )
                .on_press(Message::WeeklyReviewPressed),
            )
            .into()
    }
}

struct GoalPageWidget {
    goal_entries: Vec<GoalWidget>,
}

impl GoalPageWidget {
    fn new() -> Self {
        GoalPageWidget {
            goal_entries: Vec::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::LoadGoalsPressed => {
                let mut loaded_goals = Vec::new();
                goals::load_goals(&mut loaded_goals);

                while loaded_goals.len() > 0 {
                    if let Some(loaded_goal) = loaded_goals.pop() {
                        self.goal_entries.push(GoalWidget::new(loaded_goal));
                    }
                }
            }
            _ => {}
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
    goal: goals::Goal,
    text_color: iced::Color,
}

const CALIBRI_FONT: Font = Font::External {
    name: "Calibri",
    bytes: include_bytes!("C:/Windows/fonts/calibri.ttf"), // todo: relative?
};

impl GoalWidget {
    fn new(new_goal: goals::Goal) -> Self {
        GoalWidget {
            goal: new_goal,
            text_color: Color::from_rgb8(100, 149, 237),
        }
    }

    fn view(&mut self) -> Element<Message> {
        Container::new(
            Row::new()
                .spacing(10)
                .push(
                    Text::new(self.goal.text.to_string())
                        .size(28)
                        .font(CALIBRI_FONT)
                        .color(self.text_color),
                )
                .push(
                    Text::new(self.goal.period.to_string())
                        .size(28)
                        .font(CALIBRI_FONT)
                        .color(self.text_color),
                )
                .push(
                    Text::new(self.goal.priority.to_string())
                        .size(28)
                        .font(CALIBRI_FONT)
                        .color(self.text_color),
                )
                .push(
                    Text::new(self.goal.smart_flags.to_string())
                        .size(28)
                        .font(CALIBRI_FONT)
                        .color(self.text_color),
                )
                .push(
                    Text::new(self.goal.status.to_string())
                        .size(28)
                        .font(CALIBRI_FONT)
                        .color(self.text_color),
                )
                .push(
                    Text::new(self.goal.notes.to_string())
                        .size(28)
                        .font(CALIBRI_FONT)
                        .color(self.text_color),
                ),
        )
        .into()
    }
}

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}
