use iced::{Color, Container, Element, Font, Row, Text};

use crate::common_enums::Message;
use crate::goals;

pub struct GoalWidget {
    goal: goals::Goal,
    text_color: iced::Color,
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
        }
    }

    pub fn view(&mut self) -> Element<Message> {
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
}
