use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Creative Attributes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CreativeAttribute {
    /// Audio Ad (Auto-Play)
    AudioAdAutoPlay,
    /// Audio Ad (User Initiated)
    AudioAdUserInitiated,
    /// Expandable (Automatic)
    ExpandableAutomatic,
    /// Expandable (User Initiated - Click)
    ExpandableUserInitiatedClick,
    /// Expandable (User Initiated - Rollover)
    ExpandableUserInitiatedRollover,
    /// In-Banner Video Ad (Auto-Play)
    InBannerVideoAdAutoPlay,
    /// In-Banner Video Ad (User Initiated)
    InBannerVideoAdUserInitiated,
    /// Pop (e.g., Over, Under, or upon Exit)
    Pop,
    /// Provocative or Suggestive Imagery
    ProvocativeOrSuggestiveImagery,
    /// Shaky, Flashing, Flickering, Extreme Animation, Smileys
    ShakyFlashingFlickering,
    /// Surveys
    Surveys,
    /// Text Only
    TextOnly,
    /// User Interactive (e.g., Embedded Games)
    UserInteractive,
    /// Windows Dialog or Alert Style
    WindowsDialog,
    /// Has Audio On/Off Button
    HasAudioOnOffButton,
    /// Ad Provides Skip Button (e.g. VPAID-rendered skip button on pre-roll video)
    AdProvidesSkipButton,
    /// Adobe Flash
    AdobeFlash,
}

impl Serialize for CreativeAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            CreativeAttribute::AudioAdAutoPlay => serializer.serialize_u32(1),
            CreativeAttribute::AudioAdUserInitiated => serializer.serialize_u32(2),
            CreativeAttribute::ExpandableAutomatic => serializer.serialize_u32(3),
            CreativeAttribute::ExpandableUserInitiatedClick => serializer.serialize_u32(4),
            CreativeAttribute::ExpandableUserInitiatedRollover => serializer.serialize_u32(5),
            CreativeAttribute::InBannerVideoAdAutoPlay => serializer.serialize_u32(6),
            CreativeAttribute::InBannerVideoAdUserInitiated => serializer.serialize_u32(7),
            CreativeAttribute::Pop => serializer.serialize_u32(8),
            CreativeAttribute::ProvocativeOrSuggestiveImagery => serializer.serialize_u32(9),
            CreativeAttribute::ShakyFlashingFlickering => serializer.serialize_u32(10),
            CreativeAttribute::Surveys => serializer.serialize_u32(11),
            CreativeAttribute::TextOnly => serializer.serialize_u32(12),
            CreativeAttribute::UserInteractive => serializer.serialize_u32(13),
            CreativeAttribute::WindowsDialog => serializer.serialize_u32(14),
            CreativeAttribute::HasAudioOnOffButton => serializer.serialize_u32(15),
            CreativeAttribute::AdProvidesSkipButton => serializer.serialize_u32(16),
            CreativeAttribute::AdobeFlash => serializer.serialize_u32(17),
        }
    }
}

impl<'de> Deserialize<'de> for CreativeAttribute {
    fn deserialize<D>(deserializer: D) -> Result<CreativeAttribute, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CreativeAttribute::AudioAdAutoPlay),
            2 => Ok(CreativeAttribute::AudioAdUserInitiated),
            3 => Ok(CreativeAttribute::ExpandableAutomatic),
            4 => Ok(CreativeAttribute::ExpandableUserInitiatedClick),
            5 => Ok(CreativeAttribute::ExpandableUserInitiatedRollover),
            6 => Ok(CreativeAttribute::InBannerVideoAdAutoPlay),
            7 => Ok(CreativeAttribute::InBannerVideoAdUserInitiated),
            8 => Ok(CreativeAttribute::Pop),
            9 => Ok(CreativeAttribute::ProvocativeOrSuggestiveImagery),
            10 => Ok(CreativeAttribute::ShakyFlashingFlickering),
            11 => Ok(CreativeAttribute::Surveys),
            12 => Ok(CreativeAttribute::TextOnly),
            13 => Ok(CreativeAttribute::UserInteractive),
            14 => Ok(CreativeAttribute::WindowsDialog),
            15 => Ok(CreativeAttribute::HasAudioOnOffButton),
            16 => Ok(CreativeAttribute::AdProvidesSkipButton),
            17 => Ok(CreativeAttribute::AdobeFlash),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid CreativeAttribute value: {}",
                value
            ))),
        }
    }
}