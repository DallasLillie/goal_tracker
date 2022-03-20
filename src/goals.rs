use std::error::Error;
use std::fmt;

use bitflags::bitflags;
use chrono::NaiveDate;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use uuid::Uuid;

extern crate serde;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoalProgressType {
    DoneOrNot(bool),
    DoXManyTimes(
        (
            u16, // current_progress, 0-4, 0-31, 0-52, 0-356
            u8,  // required_completion_percentage, 0-100
        ),
    ),
}

impl Default for GoalProgressType {
    fn default() -> GoalProgressType {
        GoalProgressType::DoneOrNot(false)
    }
}

impl Serialize for GoalProgressType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            GoalProgressType::DoneOrNot(is_done) => {
                let progress_type = 0; // first byte
                let is_done = *is_done as u32; // second/third/fourth bytes
                let packed_data: u32 = progress_type | (is_done << 8);
                serializer.serialize_u32(packed_data)
            }
            GoalProgressType::DoXManyTimes(progress_and_completion) => {
                let progress_type = 1; // first byte
                let progress = progress_and_completion.0 as u32; // second and third byte
                let completion = progress_and_completion.1 as u32; // fourth byte
                let packed_data: u32 = progress_type | (progress << 8) | (completion << 24);
                serializer.serialize_u32(packed_data)
            }
        }
    }
}

impl<'de> Deserialize<'de> for GoalProgressType {
    fn deserialize<D>(deserializer: D) -> Result<GoalProgressType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let packed_data = u32::deserialize(deserializer)?;
        let progress_type = packed_data & 0xFF;
        match progress_type {
            0 => {
                let is_done = (packed_data & 0xFF00) >> 8;
                Ok(GoalProgressType::DoneOrNot(is_done != 0))
            }
            1 => {
                let progress = (packed_data & 0xFFFF00) >> 8;
                let completion = (packed_data & 0xFF000000) >> 24;
                Ok(GoalProgressType::DoXManyTimes((
                    progress as u16,
                    completion as u8,
                )))
            }
            _ => Err(serde::de::Error::custom(format!(
                "Couldn't deserialize Goal Progress Type: {}",
                packed_data
            ))),
        }
    }
}

impl fmt::Display for GoalProgressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GoalProgressType::DoneOrNot(_) => f.write_str("DoneOrNot"),
            GoalProgressType::DoXManyTimes(_) => f.write_str("DoXManyTimes"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum GoalStatus {
    InProgress,
    Successful,
    Failed,
    Retired,
}

impl Default for GoalStatus {
    fn default() -> GoalStatus {
        GoalStatus::InProgress
    }
}

impl fmt::Display for GoalStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GoalStatus::InProgress => f.write_str("InProgress"),
            GoalStatus::Successful => f.write_str("Successful"),
            GoalStatus::Failed => f.write_str("Failed"),
            GoalStatus::Retired => f.write_str("Retired"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum GoalPriority {
    Top,
    High,
    Middle,
    Low,
    Bottom,
}

impl Default for GoalPriority {
    fn default() -> GoalPriority {
        GoalPriority::Middle
    }
}

impl fmt::Display for GoalPriority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GoalPriority::Top => f.write_str("Top"),
            GoalPriority::High => f.write_str("High"),
            GoalPriority::Middle => f.write_str("Middle"),
            GoalPriority::Low => f.write_str("Low"),
            GoalPriority::Bottom => f.write_str("Bottom"),
        }
    }
}

bitflags! {
    pub struct GoalSmartFlags: u8 {
        const SPECIFIC      = 0b00000001;
        const MEASURABLE    = 0b00000010;
        const ACHIEVABLE    = 0b00000100;
        const RELEVANT      = 0b00001000;
        const TIME_BOUND    = 0b00010000;
        const SMART = Self::SPECIFIC.bits | Self::MEASURABLE.bits | Self::ACHIEVABLE.bits | Self::RELEVANT.bits | Self::TIME_BOUND.bits; // todo: might be able to use .all here
    }
}

impl fmt::Display for GoalSmartFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.bits())
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Goal {
    // todo: im curious about the size of the struct
    pub uuid: Uuid, // todo: this probably shouldn't be public
    pub text: String,
    pub start_date: NaiveDate,
    pub due_date: NaiveDate,
    pub priority: GoalPriority,
    pub smart_flags: GoalSmartFlags,
    pub progress_type: GoalProgressType,
    pub status: GoalStatus,
    pub notes: String,
    pub parent: Option<Uuid>,
}

// todo: this function feels weird here. it's a pretty UI based func, but it is implemented in this file that doesn't strictly feel tied to UI
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
