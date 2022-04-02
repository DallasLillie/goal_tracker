use std::future;

use iced::{button, Button, Column, Command, Container, Element, Row, Text};

use crate::common_enums::{ApplicationPage, Message};
use crate::edit_goal_widget;
use crate::goals;

pub struct EditGoalPageWidget {
    confirm_edit_goal_button_state: button::State,
    cancel_edit_goal_button_state: button::State,
    edit_goal_widget: edit_goal_widget::EditGoalWidget,

    goals: Vec<goals::Goal>,
}

impl EditGoalPageWidget {
    pub fn new() -> Self {
        EditGoalPageWidget {
            confirm_edit_goal_button_state: button::State::new(),
            cancel_edit_goal_button_state: button::State::new(),
            edit_goal_widget: edit_goal_widget::EditGoalWidget::new(),
            goals: Vec::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EditGoalPageCancelPressed => {
                self.edit_goal_widget = Default::default();
                Command::perform(future::ready(()), |_| {
                    Message::ChangePage(ApplicationPage::HomePage)
                })
            }
            Message::EditGoalPageConfirmPressed => {
                let new_goal: goals::Goal = self.edit_goal_widget.get_goal();

                self.edit_goal_widget = Default::default();

                let goal_edited_command = Command::perform(future::ready(()), move |_| {
                    Message::GoalEdited(new_goal.clone())
                });

                let change_page_command = Command::perform(future::ready(()), |_| {
                    Message::ChangePage(ApplicationPage::HomePage)
                });
                Command::batch([goal_edited_command, change_page_command])
            }
            _ => self.edit_goal_widget.update(message),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Edit New Goal"))
            .push(self.edit_goal_widget.view())
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.cancel_edit_goal_button_state, Text::new("Cancel"))
                            .on_press(Message::EditGoalPageCancelPressed),
                    )
                    .push(
                        Button::new(
                            &mut self.confirm_edit_goal_button_state,
                            Text::new("Confirm"),
                        )
                        .on_press(Message::EditGoalPageConfirmPressed),
                    ),
            );
        Container::new(content).into()
    }

    pub fn set_goal(&mut self, goal: goals::Goal) {
        self.edit_goal_widget.set_goal(goal, &self.goals);
    }

    pub fn set_goals(&mut self, goals: &Vec<goals::Goal>) {
        self.goals = goals.to_vec();
    }
}
