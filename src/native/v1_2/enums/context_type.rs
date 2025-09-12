use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Context Type IDs - OpenRTB Native 1.2 Section 7.1
/// 
/// The context in which the ad appears - what type of content is surrounding the ad 
/// on the page at a high level. This maps directly to the new Deep Dive on In-Feed Ad Units. 
/// This denotes the primary context, but does not imply other content may not exist on the 
/// page - for example it's expected that most content platforms have some social components, etc.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ContextType {
    /// Content-centric context such as newsfeed, article, image gallery, video gallery, or similar
    Content = 1,
    /// Social-centric context such as social network feed, email, chat, or similar
    Social = 2,
    /// Product context such as product listings, details, recommendations, reviews, or similar
    Product = 3,
    /// Exchange-specific value (500+)
    ExchangeSpecific(u32),
}

impl Serialize for ContextType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ContextType::Content => serializer.serialize_u32(1),
            ContextType::Social => serializer.serialize_u32(2),
            ContextType::Product => serializer.serialize_u32(3),
            ContextType::ExchangeSpecific(val) => serializer.serialize_u32(val),
        }
    }
}

impl<'de> Deserialize<'de> for ContextType {
    fn deserialize<D>(deserializer: D) -> Result<ContextType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(match value {
            1 => ContextType::Content,
            2 => ContextType::Social,
            3 => ContextType::Product,
            v if v >= 500 => ContextType::ExchangeSpecific(v),
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid ContextType value: {}",
                    value
                )))
            }
        })
    }
}