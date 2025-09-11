use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Playback Methods
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackMethod {
    /// Initiates on Page Load with Sound On
    AutoPlaySoundOn,
    /// Initiates on Page Load with Sound Off by Default
    AutoPlaySoundOff,
    /// Initiates on Click with Sound On
    ClickToPlay,
    /// Initiates on Mouse-Over with Sound On
    MouseOver,
    /// Initiates on Entering Viewport with Sound On
    EnterViewportSoundOn,
    /// Initiates on Entering Viewport with Sound Off by Default
    EnterViewportSoundOff,
}

impl Serialize for PlaybackMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            PlaybackMethod::AutoPlaySoundOn => serializer.serialize_u32(1),
            PlaybackMethod::AutoPlaySoundOff => serializer.serialize_u32(2),
            PlaybackMethod::ClickToPlay => serializer.serialize_u32(3),
            PlaybackMethod::MouseOver => serializer.serialize_u32(4),
            PlaybackMethod::EnterViewportSoundOn => serializer.serialize_u32(5),
            PlaybackMethod::EnterViewportSoundOff => serializer.serialize_u32(6),
        }
    }
}

impl<'de> Deserialize<'de> for PlaybackMethod {
    fn deserialize<D>(deserializer: D) -> Result<PlaybackMethod, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(PlaybackMethod::AutoPlaySoundOn),
            2 => Ok(PlaybackMethod::AutoPlaySoundOff),
            3 => Ok(PlaybackMethod::ClickToPlay),
            4 => Ok(PlaybackMethod::MouseOver),
            5 => Ok(PlaybackMethod::EnterViewportSoundOn),
            6 => Ok(PlaybackMethod::EnterViewportSoundOff),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid PlaybackMethod value: {}",
                value
            ))),
        }
    }
}