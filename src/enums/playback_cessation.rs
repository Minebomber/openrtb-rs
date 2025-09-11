use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Playback Cessation Modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackCessationMode {
    /// On Video Completion or when Terminated by User
    VideoCompletionOrUserTerminated,
    /// On Leaving Viewport or when Terminated by User
    LeavingViewportOrUserTerminated,
    /// On Leaving Viewport Continues as a Floating/Slider Unit until Video Completion or when Terminated by User
    LeavingViewportFloatingUnit,
}

impl Serialize for PlaybackCessationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            PlaybackCessationMode::VideoCompletionOrUserTerminated => serializer.serialize_u32(1),
            PlaybackCessationMode::LeavingViewportOrUserTerminated => serializer.serialize_u32(2),
            PlaybackCessationMode::LeavingViewportFloatingUnit => serializer.serialize_u32(3),
        }
    }
}

impl<'de> Deserialize<'de> for PlaybackCessationMode {
    fn deserialize<D>(deserializer: D) -> Result<PlaybackCessationMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(PlaybackCessationMode::VideoCompletionOrUserTerminated),
            2 => Ok(PlaybackCessationMode::LeavingViewportOrUserTerminated),
            3 => Ok(PlaybackCessationMode::LeavingViewportFloatingUnit),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid PlaybackCessationMode value: {}",
                value
            ))),
        }
    }
}