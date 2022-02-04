use std::error::Error;

use bitflags::bitflags;

use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

extern crate serde;

// todo: this might be better off as a startDate/endDate
#[derive(Debug, Serialize, Deserialize)]
enum GoalPeriod {
    Year,
    Month,
    Week,
    Day,
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
    pub struct GoalSmartFlags: u8 {
        const SPECIFIC      = 0b00000001;
        const MEASURABLE    = 0b00000010;
        const ACHIEVABLE    = 0b00000100;
        const RELEVANT      = 0b00001000;
        const TIME_BOUND    = 0b00010000;
        const SMART = Self::SPECIFIC.bits | Self::MEASURABLE.bits | Self::ACHIEVABLE.bits | Self::RELEVANT.bits | Self::TIME_BOUND.bits; // todo: might be able to use .all here

        // todo: kinda want a display func for this
    }
}

impl Serialize for GoalSmartFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

// todo: i have no idea why i need these lifetimes
impl<'de> Deserialize<'de> for GoalSmartFlags {
    fn deserialize<D>(deserializer: D) -> Result<GoalSmartFlags, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u8::deserialize(deserializer)?;
        GoalSmartFlags::from_bits(bits).ok_or(serde::de::Error::custom(format!(
            "Couldn't deserialize smart flags: {}",
            bits
        )))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Goal {
    // todo: add float for percentage...maybe it's in an enum struct for if the goal has number measurements
    // todo: im curious about the size of the struct
    uuid: Uuid,
    pub text: String,
    period: GoalPeriod,
    priority: GoalPriority,
    smart_flags: GoalSmartFlags,
    status: GoalStatus,
    notes: String,
    parent: Option<Uuid>,
}

impl Goal {
    pub fn is_smart(&self) -> bool {
        return self.smart_flags.bits() == GoalSmartFlags::SMART.bits();
    }
}

// todo: maybe make this function take an enum to designate which file type we're saving to?
// todo: this would be a good function to implement some tests for practice
pub fn save_goals(goals: &[Goal]) -> Result<(), Box<dyn Error>> {
    // todo: environment variable for an abs path?
    // todo: some kind of switch to enable debug mode that uses this?
    let file_path = "..\\resources\\save_test.csv";

    // todo: the docs for the csv crate say this:
    // "Note that we do not wrap the File in a buffer. The CSV reader does buffering internally,
    // so there's no need for the caller to do it"
    // im not 100% sure what that's talking about but it's probably important
    let mut writer = csv::Writer::from_path(file_path)?; // todo: atomic write

    for goal in goals {
        writer.serialize(goal)?;
    }

    writer.flush()?;
    Ok(())
}

pub fn load_goals(goals: &mut Vec<Goal>) -> Result<(), Box<dyn Error>> {
    let file_path = "..\\resources\\load_test.csv";
    let mut reader = csv::Reader::from_path(file_path)?;
    for result in reader.deserialize() {
        let goal: Goal = result?;
        goals.push(goal);
    }

    Ok(())
}
