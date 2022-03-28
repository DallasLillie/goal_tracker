use std::future;

use iced::{button, Button, Color, Command, Container, Element, Font, Row, Text};

use crate::common_enums::{ApplicationPage, Message};
use crate::goals;

pub struct GoalWidget {
    goal: goals::Goal,
    text_color: iced::Color,
    edit_goal_button_state: button::State,
    delete_goal_button_state: button::State,
}

const CALIBRI_FONT: Font = Font::External {
    name: "Calibri",
    bytes: include_bytes!("C:/Windows/fonts/calibri.ttf"), // todo: relative?
};

impl GoalWidget {
    pub fn new(new_goal: goals::Goal) -> Self {
        GoalWidget {
            goal: new_goal,
            text_color: Color::from_rgb8(100, 149, 237),
            edit_goal_button_state: button::State::new(),
            delete_goal_button_state: button::State::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EditGoalPressed(goal_uuid) => {
                if goal_uuid == self.goal.uuid {
                    let edit_goal = self.goal.clone(); // todo: what the heck is going on here.
                    Command::perform(future::ready(()), move |_| {
                        Message::ChangePage(ApplicationPage::EditGoalPage(edit_goal.clone()))
                    })
                } else {
                    Command::none()
                }
            }
            Message::GoalEdited(edited_goal) => {
                if edited_goal.uuid == self.goal.uuid {
                    self.goal = edited_goal;
                }
                Command::none()
            }
            _ => Command::none(),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Container::new(
            Row::new()
                .spacing(10)
                .push(
                    Button::new(&mut self.edit_goal_button_state, Text::new("Edit"))
                        .on_press(Message::EditGoalPressed(self.goal.uuid)),
                )
                .push(
                    Button::new(&mut self.delete_goal_button_state, Text::new("Delete"))
                        .on_press(Message::DeleteGoalPressed(self.goal.uuid)),
                )
                .push(
                    Text::new(self.goal.text.to_string())
                        .size(28)
                        .font(CALIBRI_FONT)
                        .color(self.text_color),
                )
                .push(
                    Text::new(self.goal.start_date.to_string())
                        .size(28)
                        .font(CALIBRI_FONT)
                        .color(self.text_color),
                )
                .push(
                    Text::new(self.goal.due_date.to_string())
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
                    Text::new(self.goal.progress_type.to_string())
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

    pub fn get_uuid(&self) -> uuid::Uuid {
        self.goal.uuid
    }
}
