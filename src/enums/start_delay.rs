use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Start Delay
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StartDelay {
    /// Pre-Roll
    PreRoll,
    /// Generic Mid-Roll
    GenericMidRoll,
    /// Generic Post-Roll
    GenericPostRoll,
}

impl Serialize for StartDelay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            StartDelay::PreRoll => serializer.serialize_u32(0),
            StartDelay::GenericMidRoll => serializer.serialize_u32(u32::MAX), // represents -1
            StartDelay::GenericPostRoll => serializer.serialize_u32(u32::MAX - 1), // represents -2
        }
    }
}

impl<'de> Deserialize<'de> for StartDelay {
    fn deserialize<D>(deserializer: D) -> Result<StartDelay, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(StartDelay::PreRoll),
            u32::MAX => Ok(StartDelay::GenericMidRoll), // represents -1
            4294967294 => Ok(StartDelay::GenericPostRoll), // u32::MAX - 1, represents -2
            _ => Err(serde::de::Error::custom(format!(
                "Invalid StartDelay value: {}",
                value
            ))),
        }
    }
}