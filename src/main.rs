// todo: i don't know what casing rust prefers
// todo: i'd actually like to throw some testing around. I'm thinking of making this entire thing workable from the commandline
// and then the gui stuff is just nice to throw on top. that should allow me plenty of tests i can run
// on the base functionality
use std::error::Error;
use std::process;

use bitflags::bitflags;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
enum GoalType {
    Annual,
    Monthly,
    Weekly,
    Daily,
}

#[derive(Debug, Serialize, Deserialize)]
enum GoalStatus {
    InProgress,
    Successful,
    Failed,
    Retired,
}

#[derive(Debug, Serialize, Deserialize)]
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

// todo: i have no idea why i need these lifetimes
impl<'de> Deserialize<'de> for SmartGoalFlags {
    fn deserialize<D>(deserializer: D) -> Result<SmartGoalFlags, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u8::deserialize(deserializer)?;
        SmartGoalFlags::from_bits(bits).ok_or(serde::de::Error::custom(format!(
            "Couldn't deserialize smart flags: {}",
            bits
        )))
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

// todo: maybe make this function take an enum to designate which file type we're saving to?
fn save_goals(goals: &[Goal]) -> Result<(), Box<dyn Error>> {
    // todo: environment variable for an abs path?
    // todo: some kind of switch to enable debug mode that uses this?
    let file_path = "..\\resources\\save_test.csv";

    // todo: the docs for the csv crate say this:
    // "Note that we do not wrap the File in a buffer. The CSV reader does buffering internally,
    // so there's no need for the caller to do it"
    // im not 100% sure what that's talking about but it's probably important
    let mut wtr = csv::Writer::from_path(file_path)?; // todo: atomic write

    for goal in goals {
        wtr.serialize(goal)?;
    }

    wtr.flush()?;
    Ok(())
}

// todo: need result returns here i think...
fn load_goals(goals: &mut Vec<Goal>) -> Result<(), Box<dyn Error>> {
    let file_path = "..\\resources\\load_test.csv";
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.deserialize() {
        let goal: Goal = result?;
        goals.push(goal);
    }

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut goals = Vec::new();

    load_goals(&mut goals)?;
    save_goals(&goals)
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
