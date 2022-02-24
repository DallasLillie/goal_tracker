#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug, Clone, Copy)]
pub enum ApplicationPage {
    HomePage,
    CreateGoalPage,
}
