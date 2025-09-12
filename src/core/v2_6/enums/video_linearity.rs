use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Video Linearity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VideoLinearity {
    /// Linear / In-Stream
    Linear,
    /// Non-Linear / Overlay
    NonLinear,
}

impl Serialize for VideoLinearity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            VideoLinearity::Linear => serializer.serialize_u32(1),
            VideoLinearity::NonLinear => serializer.serialize_u32(2),
        }
    }
}

impl<'de> Deserialize<'de> for VideoLinearity {
    fn deserialize<D>(deserializer: D) -> Result<VideoLinearity, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(VideoLinearity::Linear),
            2 => Ok(VideoLinearity::NonLinear),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid VideoLinearity value: {}",
                value
            ))),
        }
    }
}