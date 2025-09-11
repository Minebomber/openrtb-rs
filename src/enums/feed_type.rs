use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Feed Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeedType {
    /// Music Service
    MusicService,
    /// FM/AM Broadcast
    FmAmBroadcast,
    /// Podcast
    Podcast,
}

impl Serialize for FeedType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            FeedType::MusicService => serializer.serialize_u32(1),
            FeedType::FmAmBroadcast => serializer.serialize_u32(2),
            FeedType::Podcast => serializer.serialize_u32(3),
        }
    }
}

impl<'de> Deserialize<'de> for FeedType {
    fn deserialize<D>(deserializer: D) -> Result<FeedType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(FeedType::MusicService),
            2 => Ok(FeedType::FmAmBroadcast),
            3 => Ok(FeedType::Podcast),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid FeedType value: {}",
                value
            ))),
        }
    }
}