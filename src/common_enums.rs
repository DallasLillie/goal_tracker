use crate::goals;

#[derive(Debug, Clone)]
pub enum Message {
    LoadGoalsPressed,
    SaveGoalsPressed,
    CreateGoalPressed,
    YearlyReviewPressed,
    MonthlyReviewPressed,
    WeeklyReviewPressed,
    ChangePage(ApplicationPage),
    CreateGoalPageCreateGoalPressed, // todo: kind of annoying tracking every button pressed message for the whole program in this one message enum
    CreateGoalPageCancelPressed,
    EditGoalWidgetGoalPriorityPicked(goals::GoalPriority),
    EditGoalWidgetGoalStatusPicked(goals::GoalStatus),
    EditGoalWidgetGoalProgressTypePicked(goals::GoalProgressType),
    EditGoalWidgetGoalProgressTypeDoneOrNotCheckboxToggled(bool),
    EditGoalWidgetGoalProgressTypeCurrentProgressEntryChanged(String),
    EditGoalWidgetGoalProgressTypeRequiredCompletionEntryChanged(String),
    EditGoalWidgetSpecificCheckboxToggled(bool),
    EditGoalWidgetMeasurableCheckboxToggled(bool),
    EditGoalWidgetAchievableCheckboxToggled(bool),
    EditGoalWidgetRelevantCheckboxToggled(bool),
    EditGoalWidgetTimeboundCheckboxToggled(bool),
    EditGoalWidgetNotesEntryChanged(String),
    EditGoalWidgetGoalEntryChanged(String),
    EditGoalWidgetStartDateMonthPicked(i8),
    EditGoalWidgetStartDateDayPicked(i8),
    EditGoalWidgetStartDateYearPicked(i16),
    EditGoalWidgetEndDateMonthPicked(i8),
    EditGoalWidgetEndDateDayPicked(i8),
    EditGoalWidgetEndDateYearPicked(i16),
}

#[derive(Debug, Clone, Copy)]
pub enum ApplicationPage {
    HomePage,
    CreateGoalPage,
}
