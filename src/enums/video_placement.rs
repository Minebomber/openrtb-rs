use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Video Placement Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VideoPlacementType {
    /// In-Stream: Played before, during or after the streaming video content that the consumer has requested (Pre-roll, Mid-roll, Post-roll).
    InStream,
    /// In-Banner: Exists within a web banner that leverages the banner space to deliver a video experience as opposed to another static or rich media format.
    InBanner,
    /// In-Article: Loads and plays dynamically between paragraphs of editorial content; existing as a standalone branded message.
    InArticle,
    /// In-Feed: Found in content, social, or product feeds.
    InFeed,
    /// Interstitial/Slider/Floating: Covers the entire or a portion of screen area, but is always on screen while displayed (i.e. cannot be scrolled out of view).
    InterstitialSliderFloating,
}

impl Serialize for VideoPlacementType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            VideoPlacementType::InStream => serializer.serialize_u32(1),
            VideoPlacementType::InBanner => serializer.serialize_u32(2),
            VideoPlacementType::InArticle => serializer.serialize_u32(3),
            VideoPlacementType::InFeed => serializer.serialize_u32(4),
            VideoPlacementType::InterstitialSliderFloating => serializer.serialize_u32(5),
        }
    }
}

impl<'de> Deserialize<'de> for VideoPlacementType {
    fn deserialize<D>(deserializer: D) -> Result<VideoPlacementType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(VideoPlacementType::InStream),
            2 => Ok(VideoPlacementType::InBanner),
            3 => Ok(VideoPlacementType::InArticle),
            4 => Ok(VideoPlacementType::InFeed),
            5 => Ok(VideoPlacementType::InterstitialSliderFloating),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid VideoPlacementType value: {}",
                value
            ))),
        }
    }
}