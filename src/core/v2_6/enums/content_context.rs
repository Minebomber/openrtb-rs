use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Content Context
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContentContext {
    /// Video (i.e., video file or stream such as Internet TV broadcasts)
    Video,
    /// Game (i.e., an interactive software game)
    Game,
    /// Music (i.e., audio file or stream such as Internet radio broadcasts)
    Music,
    /// Application (i.e., an interactive software application)
    Application,
    /// Text (i.e., primarily textual document such as a web page, eBook, or news article)
    Text,
    /// Other (i.e., none of the other categories applies)
    Other,
    /// Unknown
    Unknown,
}

impl Serialize for ContentContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ContentContext::Video => serializer.serialize_u32(1),
            ContentContext::Game => serializer.serialize_u32(2),
            ContentContext::Music => serializer.serialize_u32(3),
            ContentContext::Application => serializer.serialize_u32(4),
            ContentContext::Text => serializer.serialize_u32(5),
            ContentContext::Other => serializer.serialize_u32(6),
            ContentContext::Unknown => serializer.serialize_u32(7),
        }
    }
}

impl<'de> Deserialize<'de> for ContentContext {
    fn deserialize<D>(deserializer: D) -> Result<ContentContext, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(ContentContext::Video),
            2 => Ok(ContentContext::Game),
            3 => Ok(ContentContext::Music),
            4 => Ok(ContentContext::Application),
            5 => Ok(ContentContext::Text),
            6 => Ok(ContentContext::Other),
            7 => Ok(ContentContext::Unknown),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid ContentContext value: {}",
                value
            ))),
        }
    }
}