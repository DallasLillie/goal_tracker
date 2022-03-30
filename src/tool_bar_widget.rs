use std::future;

use iced::button;
use iced::Button;
use iced::Command;
use iced::Element;
use iced::Row;
use iced::Text;

use crate::common_enums::{ApplicationPage, Message};

pub struct ToolBarWidget {
    load_goals_button_state: button::State,
    save_goals_button_state: button::State,
    create_goal_button_state: button::State,
    yearly_review_button_state: button::State,
    monthly_review_button_state: button::State,
    weekly_review_button_state: button::State,
}

impl ToolBarWidget {
    pub fn new() -> Self {
        ToolBarWidget {
            load_goals_button_state: button::State::new(),
            save_goals_button_state: button::State::new(),
            create_goal_button_state: button::State::new(),
            yearly_review_button_state: button::State::new(),
            monthly_review_button_state: button::State::new(),
            weekly_review_button_state: button::State::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CreateGoalPressed => Command::perform(
                future::ready(()), // can't find a way to send a message without a future involved
                |_| Message::ChangePage(ApplicationPage::CreateGoalPage),
            ),
            Message::LoadGoalsPressed => {
                let relative_path = Some("resources");
                let result = nfd::open_file_dialog(None, relative_path).unwrap(); // todo: what errors are hidden here?
                match result {
                    nfd::Response::Okay(file_path) => {
                        Command::perform(future::ready(()), move |_| {
                            Message::LoadGoals(file_path.to_owned())
                        })
                    }
                    _ => Command::none(),
                }
            }
            Message::SaveGoalsPressed => {
                let relative_path = Some("resources");
                let result = nfd::open_save_dialog(None, relative_path).unwrap();
                match result {
                    nfd::Response::Okay(file_path) => {
                        Command::perform(future::ready(()), move |_| {
                            Message::SaveGoals(file_path.to_owned())
                        })
                    }
                    _ => Command::none(),
                }
            }
            _ => Command::none(),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
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
