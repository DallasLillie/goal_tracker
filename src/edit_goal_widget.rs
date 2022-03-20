use std::convert::TryInto;

use chrono::{Datelike, Local, NaiveDate};
use iced::{
    pick_list, text_input, Checkbox, Column, Command, Element, PickList, Row, Text, TextInput,
};
use uuid::Uuid;

use crate::common_enums::Message;
use crate::goals;

impl goals::GoalPriority {
    const ALL: [goals::GoalPriority; 5] = [
        goals::GoalPriority::Top,
        goals::GoalPriority::High,
        goals::GoalPriority::Middle,
        goals::GoalPriority::Low,
        goals::GoalPriority::Bottom,
    ];
}

impl goals::GoalStatus {
    const ALL: [goals::GoalStatus; 4] = [
        goals::GoalStatus::InProgress,
        goals::GoalStatus::Successful,
        goals::GoalStatus::Failed,
        goals::GoalStatus::Retired,
    ];
}

impl goals::GoalProgressType {
    const ALL: [goals::GoalProgressType; 2] = [
        goals::GoalProgressType::DoneOrNot(false),
        goals::GoalProgressType::DoXManyTimes((0, 0)),
    ];
}
pub struct EditGoalWidget {
    // goal text
    goal_text_input_state: text_input::State,
    goal_text_input_entry: String,

    // start date
    start_date_month_pick_list_state: pick_list::State<i8>,
    start_date_selected_month: Option<i8>,
    start_date_day_pick_list_state: pick_list::State<i8>,
    start_date_selected_day: Option<i8>,
    start_date_year_pick_list_state: pick_list::State<i16>,
    start_date_selected_year: Option<i16>,

    // end date
    end_date_month_pick_list_state: pick_list::State<i8>,
    end_date_selected_month: Option<i8>,
    end_date_day_pick_list_state: pick_list::State<i8>,
    end_date_selected_day: Option<i8>,
    end_date_year_pick_list_state: pick_list::State<i16>,
    end_date_selected_year: Option<i16>,

    // GoalPriority
    priority_pick_list_state: pick_list::State<goals::GoalPriority>,
    selected_priority: Option<goals::GoalPriority>,

    // GoalSmartFlags
    specific_checkbox_is_checked: bool,
    measurable_checkbox_is_checked: bool,
    achievable_checkbox_is_checked: bool,
    relevant_checkbox_is_checked: bool,
    timebound_checkbox_is_checked: bool,

    // GoalProgressType
    progress_type_pick_list_state: pick_list::State<goals::GoalProgressType>,
    selected_progress_type: Option<goals::GoalProgressType>,
    done_or_not_checkbox_checked: bool,
    do_x_many_times_current_progress_text_input_state: text_input::State,
    do_x_many_times_current_progress_text_input_entry: String,
    do_x_many_times_required_completion_text_input_state: text_input::State,
    do_x_many_times_required_completion_text_input_entry: String,

    // GoalStatus
    status_pick_list_state: pick_list::State<goals::GoalStatus>,
    selected_status: Option<goals::GoalStatus>,

    // notes text
    notes_text_input_state: text_input::State,
    notes_text_input_entry: String,
    // todo: parent goal
}

impl Default for EditGoalWidget {
    fn default() -> EditGoalWidget {
        let today = Local::today();
        EditGoalWidget {
            goal_text_input_state: text_input::State::new(),
            goal_text_input_entry: "".to_owned(),
            // start date
            start_date_month_pick_list_state: pick_list::State::default(),
            start_date_selected_month: Some(today.naive_local().month().try_into().unwrap()),
            start_date_day_pick_list_state: pick_list::State::default(),
            start_date_selected_day: Some(today.naive_local().day().try_into().unwrap()),
            start_date_year_pick_list_state: pick_list::State::default(),
            start_date_selected_year: Some(today.naive_local().year().try_into().unwrap()),
            // end date
            end_date_month_pick_list_state: pick_list::State::default(),
            end_date_selected_month: Some(today.naive_local().month().try_into().unwrap()),
            end_date_day_pick_list_state: pick_list::State::default(),
            end_date_selected_day: Some(today.naive_local().day().try_into().unwrap()),
            end_date_year_pick_list_state: pick_list::State::default(),
            end_date_selected_year: Some(today.naive_local().year().try_into().unwrap()),
            // GoalPriority
            priority_pick_list_state: pick_list::State::default(),
            selected_priority: Some(goals::GoalPriority::default()),
            // GoalSmartFlags
            specific_checkbox_is_checked: true,
            measurable_checkbox_is_checked: true,
            achievable_checkbox_is_checked: true,
            relevant_checkbox_is_checked: true,
            timebound_checkbox_is_checked: true,
            // GoalProgressType
            progress_type_pick_list_state: pick_list::State::default(),
            selected_progress_type: Some(goals::GoalProgressType::default()),
            done_or_not_checkbox_checked: true,
            do_x_many_times_current_progress_text_input_state: text_input::State::new(),
            do_x_many_times_current_progress_text_input_entry: "".to_owned(),
            do_x_many_times_required_completion_text_input_state: text_input::State::new(),
            do_x_many_times_required_completion_text_input_entry: "".to_owned(),
            // GoalStatus
            status_pick_list_state: pick_list::State::default(),
            selected_status: Some(goals::GoalStatus::default()),
            // notes text
            notes_text_input_state: text_input::State::new(),
            notes_text_input_entry: "".to_owned(),
        }
    }
}

impl EditGoalWidget {
    pub fn new() -> Self {
        Self::default()
        // todo: this widget's state is kept around when going back to the home page of the application. it needs to get reset
        // todo: should take an optional goal for editing, if no goal, then it creates a default
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EditGoalWidgetGoalEntryChanged(new_text) => {
                self.goal_text_input_entry = new_text;
                Command::none()
            }
            Message::EditGoalWidgetStartDateMonthPicked(month) => {
                self.start_date_selected_month = Some(month);
                Command::none()
            }
            Message::EditGoalWidgetStartDateDayPicked(day) => {
                self.start_date_selected_day = Some(day);
                Command::none()
            }
            Message::EditGoalWidgetStartDateYearPicked(year) => {
                self.start_date_selected_year = Some(year);
                Command::none()
            }
            Message::EditGoalWidgetEndDateMonthPicked(month) => {
                self.end_date_selected_month = Some(month);
                Command::none()
            }
            Message::EditGoalWidgetEndDateDayPicked(day) => {
                self.end_date_selected_day = Some(day);
                Command::none()
            }
            Message::EditGoalWidgetEndDateYearPicked(year) => {
                self.end_date_selected_year = Some(year);
                Command::none()
            }
            Message::EditGoalWidgetGoalPriorityPicked(priority) => {
                self.selected_priority = Some(priority);
                Command::none()
            }
            Message::EditGoalWidgetSpecificCheckboxToggled(checked) => {
                self.specific_checkbox_is_checked = checked;
                Command::none()
            }
            Message::EditGoalWidgetMeasurableCheckboxToggled(checked) => {
                self.measurable_checkbox_is_checked = checked;
                Command::none()
            }
            Message::EditGoalWidgetAchievableCheckboxToggled(checked) => {
                self.achievable_checkbox_is_checked = checked;
                Command::none()
            }
            Message::EditGoalWidgetRelevantCheckboxToggled(checked) => {
                self.relevant_checkbox_is_checked = checked;
                Command::none()
            }
            Message::EditGoalWidgetTimeboundCheckboxToggled(checked) => {
                self.timebound_checkbox_is_checked = checked;
                Command::none()
            }
            Message::EditGoalWidgetGoalProgressTypePicked(progress_type) => {
                self.selected_progress_type = Some(progress_type);
                Command::none()
            }
            Message::EditGoalWidgetGoalProgressTypeDoneOrNotCheckboxToggled(checked) => {
                self.done_or_not_checkbox_checked = checked;
                Command::none()
            }
            Message::EditGoalWidgetGoalProgressTypeCurrentProgressEntryChanged(new_text) => {
                self.do_x_many_times_current_progress_text_input_entry = new_text;
                Command::none()
            }
            Message::EditGoalWidgetGoalProgressTypeRequiredCompletionEntryChanged(new_text) => {
                self.do_x_many_times_required_completion_text_input_entry = new_text;
                Command::none()
            }
            Message::EditGoalWidgetGoalStatusPicked(status) => {
                self.selected_status = Some(status);
                Command::none()
            }
            Message::EditGoalWidgetNotesEntryChanged(new_text) => {
                self.notes_text_input_entry = new_text;
                Command::none()
            }
            _ => Command::none(),
        }
    }

    // todo: would be nice to have the goal text widget auto-focused when opening create new goal page
    // todo: tab should navigate each widget
    pub fn view(&mut self) -> Element<Message> {
        let goal_text_content = Row::new().push(Text::new("Goal: ")).push(TextInput::new(
            &mut self.goal_text_input_state,
            "What is the goal?",
            &self.goal_text_input_entry,
            Message::EditGoalWidgetGoalEntryChanged,
        ));

        let start_date_content = Row::new()
            .push(Text::new("Start Date: "))
            .push(PickList::new(
                &mut self.start_date_month_pick_list_state,
                (1..12).collect::<Vec<i8>>(),
                self.start_date_selected_month,
                Message::EditGoalWidgetStartDateMonthPicked,
            ))
            .push(PickList::new(
                &mut self.start_date_day_pick_list_state,
                (1..31).collect::<Vec<i8>>(),
                self.start_date_selected_day,
                Message::EditGoalWidgetStartDateDayPicked,
            ))
            .push(PickList::new(
                &mut self.start_date_year_pick_list_state,
                (2020..2050).collect::<Vec<i16>>(),
                self.start_date_selected_year,
                Message::EditGoalWidgetStartDateYearPicked,
            ));

        let end_date_content = Row::new()
            .push(Text::new("Due Date: "))
            .push(PickList::new(
                &mut self.end_date_month_pick_list_state,
                (1..12).collect::<Vec<i8>>(),
                self.end_date_selected_month,
                Message::EditGoalWidgetEndDateMonthPicked,
            ))
            .push(PickList::new(
                &mut self.end_date_day_pick_list_state,
                (1..31).collect::<Vec<i8>>(),
                self.end_date_selected_day,
                Message::EditGoalWidgetEndDateDayPicked,
            ))
            .push(PickList::new(
                &mut self.end_date_year_pick_list_state,
                (2020..2050).collect::<Vec<i16>>(),
                self.end_date_selected_year,
                Message::EditGoalWidgetEndDateYearPicked,
            ));

        let goal_priority_content = Row::new().push(Text::new("Priority: ")).push(PickList::new(
            &mut self.priority_pick_list_state,
            &goals::GoalPriority::ALL[..],
            self.selected_priority,
            Message::EditGoalWidgetGoalPriorityPicked,
        ));

        let goal_smart_flags_content = Column::new()
            .push(Text::new("SMART: "))
            .push(Checkbox::new(
                self.specific_checkbox_is_checked,
                "Specific",
                Message::EditGoalWidgetSpecificCheckboxToggled,
            ))
            .push(Checkbox::new(
                self.measurable_checkbox_is_checked,
                "Measurable",
                Message::EditGoalWidgetMeasurableCheckboxToggled,
            ))
            .push(Checkbox::new(
                self.achievable_checkbox_is_checked,
                "Achievable",
                Message::EditGoalWidgetAchievableCheckboxToggled,
            ))
            .push(Checkbox::new(
                self.relevant_checkbox_is_checked,
                "Relevant",
                Message::EditGoalWidgetRelevantCheckboxToggled,
            ))
            .push(Checkbox::new(
                self.timebound_checkbox_is_checked,
                "Timebound",
                Message::EditGoalWidgetTimeboundCheckboxToggled,
            ));

        let mut goal_progress_type_content = Column::new().push(
            Row::new()
                .push(Text::new("Progress Type: "))
                .push(PickList::new(
                    &mut self.progress_type_pick_list_state,
                    &goals::GoalProgressType::ALL[..],
                    self.selected_progress_type,
                    Message::EditGoalWidgetGoalProgressTypePicked,
                )),
        );

        if let Some(progress_type) = self.selected_progress_type {
            match progress_type {
                goals::GoalProgressType::DoneOrNot(_) => {
                    goal_progress_type_content = goal_progress_type_content.push(Checkbox::new(
                        self.done_or_not_checkbox_checked,
                        "Done",
                        Message::EditGoalWidgetGoalProgressTypeDoneOrNotCheckboxToggled,
                    ));
                }
                goals::GoalProgressType::DoXManyTimes(_) => {
                    goal_progress_type_content = goal_progress_type_content.push(
                        Column::new()
                            .push(
                                Row::new()
                                    .push(Text::new("Current Progress: "))
                                    .push(
                                        TextInput::new(
                                            &mut self.do_x_many_times_current_progress_text_input_state,
                                            "(0-max, int)",
                                            &self.do_x_many_times_current_progress_text_input_entry,
                                            Message::EditGoalWidgetGoalProgressTypeCurrentProgressEntryChanged,
                                        ),
                                    )
                            )
                            .push(
                                Row::new()
                                    .push(Text::new("Required Completion Percentage: "))
                                    .push(
                                        TextInput::new(
                                            &mut self.do_x_many_times_required_completion_text_input_state,
                                            "(0-100, int)",
                                            &self.do_x_many_times_required_completion_text_input_entry,
                                            Message::EditGoalWidgetGoalProgressTypeRequiredCompletionEntryChanged,
                                        )
                                    )
                            ),
                    );
                }
            }
        }

        let goal_status_content = Row::new().push(Text::new("Status: ")).push(PickList::new(
            &mut self.status_pick_list_state,
            &goals::GoalStatus::ALL[..],
            self.selected_status,
            Message::EditGoalWidgetGoalStatusPicked,
        ));

        let goal_notes_content = Row::new().push(Text::new("Notes: ")).push(TextInput::new(
            &mut self.notes_text_input_state,
            "put any extra notes here",
            &self.notes_text_input_entry,
            Message::EditGoalWidgetNotesEntryChanged,
        ));

        Column::new()
            .push(goal_text_content)
            .push(start_date_content)
            .push(end_date_content)
            .push(goal_priority_content)
            .push(goal_smart_flags_content)
            .push(goal_progress_type_content)
            .push(goal_status_content)
            .push(goal_notes_content)
            .into()
    }

    pub fn get_goal(&self) -> goals::Goal {
        let goal = goals::Goal {
            uuid: Uuid::new_v4(), // todo: sort of a weird concept that get_goal does the generation of the Uuid
            text: self.goal_text_input_entry.clone(),
            start_date: NaiveDate::from_ymd(
                self.start_date_selected_year.unwrap().into(),
                self.start_date_selected_month.unwrap().try_into().unwrap(), // todo: not really sure what this is about but rust recommended it
                self.start_date_selected_day.unwrap().try_into().unwrap(),
            ),
            due_date: NaiveDate::from_ymd(
                self.end_date_selected_year.unwrap().into(),
                self.end_date_selected_month.unwrap().try_into().unwrap(),
                self.end_date_selected_day.unwrap().try_into().unwrap(),
            ),
            priority: self.selected_priority.unwrap_or_default(),
            smart_flags: goals::GoalSmartFlags::empty(),
            progress_type: match self.selected_progress_type {
                Some(progress_type) => match progress_type {
                    goals::GoalProgressType::DoneOrNot(_) => {
                        // todo: would be nice if this actually had the data in it
                        goals::GoalProgressType::DoneOrNot(self.done_or_not_checkbox_checked)
                    }
                    goals::GoalProgressType::DoXManyTimes(_) => {
                        // todo: need some kind validation on the input
                        goals::GoalProgressType::DoXManyTimes((
                            self.do_x_many_times_current_progress_text_input_entry
                                .parse::<u16>()
                                .unwrap(),
                            self.do_x_many_times_required_completion_text_input_entry
                                .parse::<u8>()
                                .unwrap(),
                        ))
                    }
                },
                None => goals::GoalProgressType::DoneOrNot(false),
            },
            status: self.selected_status.unwrap_or_default(),
            notes: self.notes_text_input_entry.clone(),
            parent: None,
        };
        return goal;
    }
}
