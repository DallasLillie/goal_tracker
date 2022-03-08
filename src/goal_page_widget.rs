use iced::{Column, Element};

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

    pub fn update(&mut self, message: Message) {
        match message {
            Message::LoadGoalsPressed => {
                // todo: should these be merged with the currently loaded goals?
                if goals::load_goals(&mut self.goals).is_ok() {
                    for goal in self.goals.iter() {
                        self.goal_entries
                            .push(goal_widget::GoalWidget::new(goals::Goal::clone(&goal)));
                    }
                } // todo: handle is_err()
            }
            Message::SaveGoalsPressed => if goals::save_goals(&self.goals).is_ok() {}, // todo: respond if is_ok/is_err
            _ => {}
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
}
