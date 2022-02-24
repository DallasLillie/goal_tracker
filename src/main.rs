use iced::{
    button, executor, Align, Application, Button, Clipboard, Color, Column, Command, Container,
    Element, Font, Length, Row, Sandbox, Scrollable, Settings, Text,
};

use std::future;

mod goals;

extern crate csv;
// this has to be in the crate root according to Rust's rules
#[macro_use]
extern crate serde_derive;

struct MyApp {
    goals: Vec<goals::Goal>,
    goal_page: GoalPageWidget,
    tool_bar: ToolBarWidget,
    create_new_goal_page: CreateNewGoalPage,
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
    ChangePage(ApplicationPage),
    CreateGoalPageCreateGoalPressed, // todo: kind of annoying tracking every button pressed message for the whole program in this one message enum
    CreateGoalPageCancelPressed,
}

#[derive(Debug, Clone, Copy)]
enum ApplicationPage {
    HomePage,
    CreateGoalPage,
}

// todo: alright this has to start being separated into other files
impl Application for MyApp {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            MyApp {
                goals: Vec::new(),
                goal_page: GoalPageWidget::new(), // todo: should probably have a "home_page" that houses this stuff
                tool_bar: ToolBarWidget::new(),
                current_page: ApplicationPage::HomePage,
                create_new_goal_page: CreateNewGoalPage::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Goal Tracker")
    }

    fn update(&mut self, message: Message, _clip_board: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ChangePage(new_page) => {
                self.current_page = new_page;
                Command::none()
            }
            _ => match self.current_page {
                ApplicationPage::HomePage => {
                    self.goal_page.update(message); // todo: need to collect commands from widgets and then send them all...which sounds bad too tbh. but something has to change here
                    self.tool_bar.update(message) // todo: i wonder how order here affects the way the messages are processed if toolbar sends a new message before goal_page has a chance to update
                }
                ApplicationPage::CreateGoalPage => self.create_new_goal_page.update(message),
            },
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        match self.current_page {
            ApplicationPage::HomePage => {
                let content = Column::new()
                    .push(Text::new("Home"))
                    .push(self.tool_bar.view())
                    .push(self.goal_page.view());
                // todo: let scrollable = Scrollable::new(&mut self.scroo).push(Container::new(content));
                Container::new(content).into()
            }
            ApplicationPage::CreateGoalPage => self.create_new_goal_page.view(),
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

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CreateGoalPressed => Command::perform(
                future::ready(()), // can't find a way to send a message without a future involved
                |_| Message::ChangePage(ApplicationPage::CreateGoalPage),
            ),
            _ => Command::none(),
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

struct CreateNewGoalPage {
    confirm_create_goal_button_state: button::State,
    cancel_create_goal_button_state: button::State,
}

impl CreateNewGoalPage {
    fn new() -> Self {
        CreateNewGoalPage {
            confirm_create_goal_button_state: button::State::new(),
            cancel_create_goal_button_state: button::State::new(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CreateGoalPageCancelPressed => Command::perform(
                future::ready(()), // can't find a way to send a message without a future involved
                |_| Message::ChangePage(ApplicationPage::HomePage),
            ),
            _ => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new().push(Text::new("Create New Goal")).push(
            Row::new()
                .push(
                    Button::new(
                        &mut self.cancel_create_goal_button_state,
                        Text::new("Cancel"),
                    )
                    .on_press(Message::CreateGoalPageCancelPressed),
                )
                .push(
                    Button::new(
                        &mut self.confirm_create_goal_button_state,
                        Text::new("Create Goal"),
                    )
                    .on_press(Message::CreateGoalPageCreateGoalPressed),
                ),
        );
        Container::new(content).into()
    }
}

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}
