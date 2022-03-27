use iced::{Column, Command, Element};

use crate::common_enums::Message;
use crate::goal_widget;
use crate::goals;

pub struct GoalPageWidget {
    goal_entries: Vec<goal_widget::GoalWidget>,
    goals: Vec<goals::Goal>,
}

impl GoalPageWidget {
    pub fn new() -> Self {
        GoalPageWidget {
            goal_entries: Vec::new(),
            goals: Vec::new(), // todo: this is only necessary for how save is implemented right now. once save creates Goal structs based off the GoalWidgets, it can be removed
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadGoalsPressed => {
                let relative_path = Some("resources");
                let result = nfd::open_file_dialog(None, relative_path) // must use backslashes
                    .unwrap(); // todo: what errors are hidden here?
                match result {
                    nfd::Response::Okay(file_path) => {
                        self.goals.clear();
                        self.goal_entries.clear();
                        // todo: would like a messagebox here on DeserializeError rather than panicing and crashing the program
                        goals::load_goals(&file_path, &mut self.goals).unwrap();
                        for goal in self.goals.iter() {
                            self.goal_entries
                                .push(goal_widget::GoalWidget::new(goals::Goal::clone(&goal)));
                        }
                    }
                    _ => {}
                }
                Command::none()
            }
            Message::SaveGoalsPressed => {
                let relative_path = Some("resources");
                let result = nfd::open_save_dialog(None, relative_path).unwrap();

                match result {
                    nfd::Response::Okay(file_path) => {
                        if goals::save_goals(&file_path, &self.goals).is_ok() {}
                        // todo: respond if is_ok/is_err
                    }
                    _ => {}
                }
                Command::none()
            }
            Message::GoalEdited(edited_goal) => {
                let to_clone_message = Message::GoalEdited(edited_goal.clone()); // todo: another spot where clones are getting out of hand
                self.update_goal(edited_goal);
                let mut commands = Vec::new();
                for goal_entry in self.goal_entries.iter_mut() {
                    let cloned_message = to_clone_message.clone();
                    commands.push(goal_entry.update(cloned_message));
                }
                Command::batch(commands)
            }
            _ => {
                let mut commands = Vec::new();
                for goal_entry in self.goal_entries.iter_mut() {
                    let cloned_message = message.clone();
                    commands.push(goal_entry.update(cloned_message));
                }
                Command::batch(commands)
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let mut content = Column::new();

        for goal_entry in self.goal_entries.iter_mut() {
            content = content.push(goal_entry.view());
        }

        content.into()
    }

    pub fn add_goal(&mut self, new_goal: goals::Goal) {
        // todo: reference here?
        self.goal_entries
            .push(goal_widget::GoalWidget::new(new_goal.clone())); // todo: so many clones. id rather avoid it when i can
        self.goals.push(new_goal);
    }

    pub fn update_goal(&mut self, updated_goal: goals::Goal) {
        for goal in self.goals.iter_mut() {
            if goal.uuid == updated_goal.uuid {
                *goal = updated_goal;
                break;
            }
        }
    }
}
