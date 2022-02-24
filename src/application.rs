use iced::{executor, Application, Clipboard, Column, Command, Container, Element, Text};

use crate::common_enums::{ApplicationPage, Message};
use crate::{create_goal_page_widget, goal_page_widget, tool_bar_widget};

pub struct MyApp {
    goal_page: goal_page_widget::GoalPageWidget,
    tool_bar: tool_bar_widget::ToolBarWidget,
    create_new_goal_page: create_goal_page_widget::CreateNewGoalPage,
    current_page: ApplicationPage,
}

// todo: alright this has to start being separated into other files
impl Application for MyApp {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            MyApp {
                goal_page: goal_page_widget::GoalPageWidget::new(), // todo: should probably have a "home_page" that houses this stuff
                tool_bar: tool_bar_widget::ToolBarWidget::new(),
                current_page: ApplicationPage::HomePage,
                create_new_goal_page: create_goal_page_widget::CreateNewGoalPage::new(),
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
