// todo: i don't know what casing rust prefers
// todo: i'd actually like to throw some testing around. I'm thinking of making this entire thing workable from the commandline
// and then the gui stuff is just nice to throw on top. that should allow me plenty of tests i can run
// on the base functionality
use std::error::Error;

use bitflags::bitflags;

use serde::ser::{Serialize, Serializer};

extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize)]
enum GoalType {
    Annual,
    Monthly,
    Weekly,
    Daily,
}

#[derive(Debug, Serialize)]
enum GoalStatus {
    InProgress,
    Successful,
    Failed,
    Retired,
}

#[derive(Debug, Serialize)]
enum GoalPriority {
    Top,
    High,
    Middle,
    Low,
    Bottom,
}

bitflags! {
    pub struct SmartGoalFlags: u8 { // todo: i don't really know how the u8 works here tbh
        const SPECIFIC      = 0b00000001;
        const MEASURABLE    = 0b00000010;
        const ACTIONABLE    = 0b00000100;
        const RELEVANT      = 0b00001000;
        const TIME_BOUND    = 0b00010000;
        const SMART = Self::SPECIFIC.bits | Self::MEASURABLE.bits | Self::ACTIONABLE.bits | Self::RELEVANT.bits | Self::TIME_BOUND.bits; // todo: might be able to use .all here

        // kinda want a display func for this
    }
}

impl Serialize for SmartGoalFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

#[derive(Debug, Serialize)]
struct Goal {
    // todo: reorg here... also, im curious about the size of the struct
    g_type: GoalType, // todo: better name
    text: String,     // todo: &str??
    status: GoalStatus,
    notes: String,
    smart_flags: SmartGoalFlags, // todo: should this just be a u8???
    priority: GoalPriority,      // todo: this might work better as a number
                                 // parent: &Goal, // todo: this says i need a lifetime or something
}
// kinda want an is_smart function

use std::io;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    let goal_test_1 = Goal {
        g_type: GoalType::Daily,
        text: String::from("g1"),
        status: GoalStatus::InProgress,
        notes: String::from(""),
        smart_flags: SmartGoalFlags::SMART,
        priority: GoalPriority::Top,
    };
    let goal_test_2 = Goal {
        g_type: GoalType::Annual,
        text: String::from("g2"),
        status: GoalStatus::Failed,
        notes: String::from("not empty"),
        smart_flags: SmartGoalFlags::SPECIFIC | SmartGoalFlags::MEASURABLE,
        priority: GoalPriority::High,
    };
    let goal_test_3 = Goal {
        g_type: GoalType::Monthly,
        text: String::from("g3"),
        status: GoalStatus::Retired,
        notes: String::from("\"comma, perhaps\""),
        smart_flags: SmartGoalFlags::RELEVANT | SmartGoalFlags::ACTIONABLE,
        priority: GoalPriority::Middle,
    };
    let goal_test_4 = Goal {
        g_type: GoalType::Weekly,
        text: String::from("g4"),
        status: GoalStatus::Successful,
        notes: String::from("quotes \"perhaps\""),
        smart_flags: SmartGoalFlags::TIME_BOUND & SmartGoalFlags::MEASURABLE,
        priority: GoalPriority::Low,
    };
    let goal_test_5 = Goal {
        g_type: GoalType::Daily,
        text: String::from("g5"),
        status: GoalStatus::InProgress,
        notes: String::from("i have notes here"),
        smart_flags: SmartGoalFlags::SMART,
        priority: GoalPriority::Bottom,
    };

    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.serialize(goal_test_1)?;
    wtr.serialize(goal_test_2)?;
    wtr.serialize(goal_test_3)?;
    wtr.serialize(goal_test_4)?;
    wtr.serialize(goal_test_5)?;

    // goal_test_s are invalid here
    wtr.flush()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
