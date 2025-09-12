use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Companion Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompanionType {
    /// Static Resource
    StaticResource,
    /// HTML Resource
    HtmlResource,
    /// iframe Resource
    IframeResource,
}

impl Serialize for CompanionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            CompanionType::StaticResource => serializer.serialize_u32(1),
            CompanionType::HtmlResource => serializer.serialize_u32(2),
            CompanionType::IframeResource => serializer.serialize_u32(3),
        }
    }
}

impl<'de> Deserialize<'de> for CompanionType {
    fn deserialize<D>(deserializer: D) -> Result<CompanionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CompanionType::StaticResource),
            2 => Ok(CompanionType::HtmlResource),
            3 => Ok(CompanionType::IframeResource),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid CompanionType value: {}",
                value
            ))),
        }
    }
}