use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Placement Type IDs - OpenRTB Native 1.2 Section 7.3
///
/// The FORMAT of the ad you are purchasing, separate from the surrounding context
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum PlacementType {
    /// In the feed of content - for example as an item inside the organic feed/grid/listing/carousel
    InFeed = 1,
    /// In the atomic unit of the content - IE in the article page or single image page
    InContent = 2,
    /// Outside the core content - for example in the ads section on the right rail, 
    /// as a banner-style placement near the content, etc.
    OutsideContent = 3,
    /// Recommendation widget, most commonly presented below the article content
    Recommendation = 4,
    /// Exchange-specific value (500+)
    ExchangeSpecific(u32),
}

impl Serialize for PlacementType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            PlacementType::InFeed => serializer.serialize_u32(1),
            PlacementType::InContent => serializer.serialize_u32(2),
            PlacementType::OutsideContent => serializer.serialize_u32(3),
            PlacementType::Recommendation => serializer.serialize_u32(4),
            PlacementType::ExchangeSpecific(val) => serializer.serialize_u32(val),
        }
    }
}

impl<'de> Deserialize<'de> for PlacementType {
    fn deserialize<D>(deserializer: D) -> Result<PlacementType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(match value {
            1 => PlacementType::InFeed,
            2 => PlacementType::InContent,
            3 => PlacementType::OutsideContent,
            4 => PlacementType::Recommendation,
            v if v >= 500 => PlacementType::ExchangeSpecific(v),
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid PlacementType value: {}",
                    value
                )))
            }
        })
    }
}