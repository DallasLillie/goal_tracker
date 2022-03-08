use std::future;

use iced::{button, Button, Column, Command, Container, Element, Row, Text};

use crate::common_enums::{ApplicationPage, Message};
use crate::edit_goal_widget;
use crate::goals;

pub struct CreateNewGoalPage {
    confirm_create_goal_button_state: button::State,
    cancel_create_goal_button_state: button::State,
    edit_goal_widget: edit_goal_widget::EditGoalWidget,
}

impl CreateNewGoalPage {
    pub fn new() -> Self {
        CreateNewGoalPage {
            confirm_create_goal_button_state: button::State::new(),
            cancel_create_goal_button_state: button::State::new(),
            edit_goal_widget: edit_goal_widget::EditGoalWidget::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CreateGoalPageCancelPressed => Command::perform(
                // todo: reset edit goal widget state
                future::ready(()), // can't find a way to send a message without a future involved
                |_| Message::ChangePage(ApplicationPage::HomePage),
            ),
            Message::CreateGoalPageCreateGoalPressed => {
                let new_goal: goals::Goal = self.edit_goal_widget.get_goal();
                let add_goal_command = Command::perform(future::ready(()), move |_| {
                    Message::NewGoalCreated(new_goal.clone())
                });

                let change_command = Command::perform(future::ready(()), |_| {
                    Message::ChangePage(ApplicationPage::HomePage)
                });
                Command::batch(vec![add_goal_command, change_command])
            }
            _ => self.edit_goal_widget.update(message),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Create New Goal"))
            .push(self.edit_goal_widget.view())
            .push(
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
