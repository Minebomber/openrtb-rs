use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Image Asset Types - OpenRTB Native 1.2 Section 7.5
///
/// Below is a list of common image asset element types of native advertising at the time
/// of writing this spec. This list is non-exhaustive and intended to be extended by the
/// buyers and sellers as the format evolves.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ImageAssetType {
    /// Icon image
    /// Optional. max height: at least 50, aspect ratio: 1:1
    Icon = 1,
    /// Large image preview for the ad
    /// At least one of 2 size variants required:
    /// Small Variant: max height: at least 200, max width: at least 200, 267, or 382
    /// aspect ratio: 1:1, 4:3, or 1.91:1
    /// Large Variant: max height: at least 627, max width: at least 627, 836, or 1198
    /// aspect ratio: 1:1, 4:3, or 1.91:1
    Main = 3,
    /// Reserved for Exchange specific usage numbered above 500
    ExchangeSpecific(u32),
}

impl Serialize for ImageAssetType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ImageAssetType::Icon => serializer.serialize_u32(1),
            ImageAssetType::Main => serializer.serialize_u32(3),
            ImageAssetType::ExchangeSpecific(val) => serializer.serialize_u32(val),
        }
    }
}

impl<'de> Deserialize<'de> for ImageAssetType {
    fn deserialize<D>(deserializer: D) -> Result<ImageAssetType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(match value {
            1 => ImageAssetType::Icon,
            3 => ImageAssetType::Main,
            v if v >= 500 => ImageAssetType::ExchangeSpecific(v),
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid ImageAssetType value: {}",
                    value
                )));
            }
        })
    }
}

