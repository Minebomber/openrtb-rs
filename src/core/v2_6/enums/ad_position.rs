use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Ad Position
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AdPosition {
    /// Unknown
    Unknown,
    /// Above the Fold
    AboveTheFold,
    /// DEPRECATED - May or may not be initially visible depending on screen size/resolution.
    MayOrMayNotBeVisible,
    /// Below the Fold
    BelowTheFold,
    /// Header
    Header,
    /// Footer
    Footer,
    /// Sidebar
    Sidebar,
    /// Full Screen
    FullScreen,
}

impl Serialize for AdPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            AdPosition::Unknown => serializer.serialize_u32(0),
            AdPosition::AboveTheFold => serializer.serialize_u32(1),
            AdPosition::MayOrMayNotBeVisible => serializer.serialize_u32(2),
            AdPosition::BelowTheFold => serializer.serialize_u32(3),
            AdPosition::Header => serializer.serialize_u32(4),
            AdPosition::Footer => serializer.serialize_u32(5),
            AdPosition::Sidebar => serializer.serialize_u32(6),
            AdPosition::FullScreen => serializer.serialize_u32(7),
        }
    }
}

impl<'de> Deserialize<'de> for AdPosition {
    fn deserialize<D>(deserializer: D) -> Result<AdPosition, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(AdPosition::Unknown),
            1 => Ok(AdPosition::AboveTheFold),
            2 => Ok(AdPosition::MayOrMayNotBeVisible),
            3 => Ok(AdPosition::BelowTheFold),
            4 => Ok(AdPosition::Header),
            5 => Ok(AdPosition::Footer),
            6 => Ok(AdPosition::Sidebar),
            7 => Ok(AdPosition::FullScreen),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid AdPosition value: {}",
                value
            ))),
        }
    }
}