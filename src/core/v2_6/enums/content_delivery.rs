use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Content Delivery Methods
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContentDeliveryMethod {
    /// Streaming
    Streaming,
    /// Progressive
    Progressive,
    /// Download
    Download,
}

impl Serialize for ContentDeliveryMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ContentDeliveryMethod::Streaming => serializer.serialize_u32(1),
            ContentDeliveryMethod::Progressive => serializer.serialize_u32(2),
            ContentDeliveryMethod::Download => serializer.serialize_u32(3),
        }
    }
}

impl<'de> Deserialize<'de> for ContentDeliveryMethod {
    fn deserialize<D>(deserializer: D) -> Result<ContentDeliveryMethod, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(ContentDeliveryMethod::Streaming),
            2 => Ok(ContentDeliveryMethod::Progressive),
            3 => Ok(ContentDeliveryMethod::Download),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid ContentDeliveryMethod value: {}",
                value
            ))),
        }
    }
}