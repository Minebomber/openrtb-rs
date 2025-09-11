use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// User Agent Source
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserAgentSource {
    /// Unknown or higher than 3
    Unknown,
    /// User agent string in User-Agent header
    UserAgentString,
    /// User agent data from User-Agent Client Hints
    UserAgentClientHints,
    /// User agent string in User-Agent header enhanced with user agent data from User-Agent Client Hints
    Enhanced,
}

impl Serialize for UserAgentSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            UserAgentSource::Unknown => serializer.serialize_u32(0),
            UserAgentSource::UserAgentString => serializer.serialize_u32(1),
            UserAgentSource::UserAgentClientHints => serializer.serialize_u32(2),
            UserAgentSource::Enhanced => serializer.serialize_u32(3),
        }
    }
}

impl<'de> Deserialize<'de> for UserAgentSource {
    fn deserialize<D>(deserializer: D) -> Result<UserAgentSource, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(UserAgentSource::Unknown),
            1 => Ok(UserAgentSource::UserAgentString),
            2 => Ok(UserAgentSource::UserAgentClientHints),
            3 => Ok(UserAgentSource::Enhanced),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid UserAgentSource value: {}",
                value
            ))),
        }
    }
}