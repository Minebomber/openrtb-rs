use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Banner Ad Type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BannerAdType {
    /// XHTML Text Ad (usually mobile)
    XhtmlTextAd,
    /// XHTML Banner Ad. (usually mobile)
    XhtmlBannerAd,
    /// JavaScript Ad; must be valid XHTML (i.e., Script Tags Included)
    JavascriptAd,
    /// Iframe
    Iframe,
}

impl Serialize for BannerAdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            BannerAdType::XhtmlTextAd => serializer.serialize_u32(1),
            BannerAdType::XhtmlBannerAd => serializer.serialize_u32(2),
            BannerAdType::JavascriptAd => serializer.serialize_u32(3),
            BannerAdType::Iframe => serializer.serialize_u32(4),
        }
    }
}

impl<'de> Deserialize<'de> for BannerAdType {
    fn deserialize<D>(deserializer: D) -> Result<BannerAdType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(BannerAdType::XhtmlTextAd),
            2 => Ok(BannerAdType::XhtmlBannerAd),
            3 => Ok(BannerAdType::JavascriptAd),
            4 => Ok(BannerAdType::Iframe),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid BannerAdType value: {}",
                value
            ))),
        }
    }
}

