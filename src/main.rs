// todo: i don't know what casing rust prefers
// todo: i'd actually like to throw some testing around. I'm thinking of making this entire thing workable from the commandline
// and then the gui stuff is just nice to throw on top. that should allow me plenty of tests i can run
// on the base functionality
use bitflags::bitflags;

enum GoalType {
    Annual,
    Monthly,
    Weekly,
    Daily,
}

enum GoalStatus {
    InProgress,
    Successful,
    Failed,
    Retired,
}

enum GoalPriority {
    Top,
    High,
    Middle,
    Low,
    Bottom,
}

bitflags! {
    struct SmartGoalFlags: u8 { // todo: i don't really know how the u8 works here tbh
        const SPECIFIC      = 0b00000001;
        const MEASURABLE    = 0b00000010;
        const ACTIONABLE    = 0b00000100;
        const RELEVANT      = 0b00001000;
        const TIME_BOUND    = 0b00010000;
         const SMART = Self::SPECIFIC.bits | Self::MEASURABLE.bits | Self::ACTIONABLE.bits | Self::RELEVANT.bits | Self::TIME_BOUND.bits; // todo: might be able to use .all here

        // kinda want a display func for this
    }
}

struct Goal {
    g_type: GoalType, // todo: better name
    text: String,     // todo: &str???
    status: GoalStatus,
    notes: String,
    smart_flags: SmartGoalFlags, // todo: should this just be a u8???
    priority: GoalPriority,      // todo: this might work better as a number
                                 // parent: &Goal, // todo: this says i need a lifetime or something
}
// kinda want an is_smart function

fn main() {
    let goals: Vec<Goal> = Vec::new();
}
