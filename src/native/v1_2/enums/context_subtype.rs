use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Context SubType IDs - OpenRTB Native 1.2 Section 7.2
/// 
/// Next-level context in which the ad appears. Again this reflects the primary context, 
/// and does not imply no presence of other elements. For example, an article is likely 
/// to contain images but is still first and foremost an article. SubType should only be 
/// combined with the primary context type as indicated (ie for a context type of 1, only 
/// context subtypes that start with 1 are valid).
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ContextSubType {
    /// General or mixed content
    GeneralContent = 10,
    /// Primarily article content (which of course could include images, etc as part of the article)
    Article = 11,
    /// Primarily video content
    Video = 12,
    /// Primarily audio content
    Audio = 13,
    /// Primarily image content
    Image = 14,
    /// User-generated content - forums, comments, etc
    UserGenerated = 15,
    /// General social content such as a general social network
    GeneralSocial = 20,
    /// Primarily email content
    Email = 21,
    /// Primarily chat/IM content
    Chat = 22,
    /// Content focused on selling products, whether digital or physical
    SellingProducts = 30,
    /// Application store/marketplace
    AppStore = 31,
    /// Product reviews site primarily (which may sell product secondarily)
    ProductReviews = 32,
    /// Exchange-specific value (500+)
    ExchangeSpecific(u32),
}

impl Serialize for ContextSubType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ContextSubType::GeneralContent => serializer.serialize_u32(10),
            ContextSubType::Article => serializer.serialize_u32(11),
            ContextSubType::Video => serializer.serialize_u32(12),
            ContextSubType::Audio => serializer.serialize_u32(13),
            ContextSubType::Image => serializer.serialize_u32(14),
            ContextSubType::UserGenerated => serializer.serialize_u32(15),
            ContextSubType::GeneralSocial => serializer.serialize_u32(20),
            ContextSubType::Email => serializer.serialize_u32(21),
            ContextSubType::Chat => serializer.serialize_u32(22),
            ContextSubType::SellingProducts => serializer.serialize_u32(30),
            ContextSubType::AppStore => serializer.serialize_u32(31),
            ContextSubType::ProductReviews => serializer.serialize_u32(32),
            ContextSubType::ExchangeSpecific(val) => serializer.serialize_u32(val),
        }
    }
}

impl<'de> Deserialize<'de> for ContextSubType {
    fn deserialize<D>(deserializer: D) -> Result<ContextSubType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(match value {
            10 => ContextSubType::GeneralContent,
            11 => ContextSubType::Article,
            12 => ContextSubType::Video,
            13 => ContextSubType::Audio,
            14 => ContextSubType::Image,
            15 => ContextSubType::UserGenerated,
            20 => ContextSubType::GeneralSocial,
            21 => ContextSubType::Email,
            22 => ContextSubType::Chat,
            30 => ContextSubType::SellingProducts,
            31 => ContextSubType::AppStore,
            32 => ContextSubType::ProductReviews,
            v if v >= 500 => ContextSubType::ExchangeSpecific(v),
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid ContextSubType value: {}",
                    value
                )))
            }
        })
    }
}