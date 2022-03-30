use std::future;

use iced::{executor, Application, Clipboard, Column, Command, Container, Element, Text};

use crate::common_enums::{ApplicationFlags, ApplicationPage, Message};
use crate::{create_goal_page_widget, edit_goal_page_widget, goal_page_widget, tool_bar_widget};

pub struct MyApp {
    goal_page: goal_page_widget::GoalPageWidget,
    tool_bar: tool_bar_widget::ToolBarWidget,
    create_new_goal_page: create_goal_page_widget::CreateNewGoalPage,
    edit_goal_page: edit_goal_page_widget::EditGoalPageWidget,
    current_page: ApplicationPage,
}

impl Application for MyApp {
    type Message = Message;
    type Flags = ApplicationFlags;
    type Executor = executor::Default;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let application = MyApp {
            goal_page: goal_page_widget::GoalPageWidget::new(), // todo: should probably have a "home_page" that houses this stuff
            tool_bar: tool_bar_widget::ToolBarWidget::new(),
            current_page: ApplicationPage::HomePage,
            create_new_goal_page: create_goal_page_widget::CreateNewGoalPage::new(),
            edit_goal_page: edit_goal_page_widget::EditGoalPageWidget::new(),
        };
        let command = match flags.startup_goals_file_path {
            Some(file_path) => Command::perform(future::ready(()), move |_| {
                Message::LoadGoals(file_path.to_owned())
            }),
            None => Command::none(),
        };
        (application, command)
    }

    fn title(&self) -> String {
        String::from("Goal Tracker")
    }

    fn update(&mut self, message: Message, _clip_board: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ChangePage(new_page) => {
                let cloned_page = new_page.clone(); // todo: so many bad clones. feels like a crutch
                match new_page {
                    ApplicationPage::EditGoalPage(goal_to_edit) => {
                        self.edit_goal_page.set_goal(goal_to_edit);
                    }
                    _ => {}
                }
                self.current_page = cloned_page;
                Command::none()
            }
            Message::NewGoalCreated(new_goal) => {
                self.goal_page.add_goal(new_goal);
                Command::none()
            }
            Message::GoalEdited(_) => {
                self.goal_page.update(message);
                Command::none()
            }
            _ => match self.current_page {
                ApplicationPage::HomePage => {
                    let cloned_message = message.clone(); // todo: cloning here seems like it could lead down a bad path in the long run
                    let goal_page_command = self.goal_page.update(message);
                    let tool_bar_command = self.tool_bar.update(cloned_message); // todo: i wonder how order here affects the way the messages are processed if toolbar sends a new message before goal_page has a chance to update
                    Command::batch([goal_page_command, tool_bar_command])
                }
                ApplicationPage::CreateGoalPage => self.create_new_goal_page.update(message),
                ApplicationPage::EditGoalPage(_) => self.edit_goal_page.update(message),
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
            ApplicationPage::EditGoalPage(_) => self.edit_goal_page.view(),
        }
    }
}
