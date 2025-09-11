use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Production Quality
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProductionQuality {
    /// Unknown
    Unknown,
    /// Professionally Produced
    Professional,
    /// Prosumer
    Prosumer,
    /// User Generated (UGC)
    UserGenerated,
}

impl Serialize for ProductionQuality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ProductionQuality::Unknown => serializer.serialize_u32(0),
            ProductionQuality::Professional => serializer.serialize_u32(1),
            ProductionQuality::Prosumer => serializer.serialize_u32(2),
            ProductionQuality::UserGenerated => serializer.serialize_u32(3),
        }
    }
}

impl<'de> Deserialize<'de> for ProductionQuality {
    fn deserialize<D>(deserializer: D) -> Result<ProductionQuality, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(ProductionQuality::Unknown),
            1 => Ok(ProductionQuality::Professional),
            2 => Ok(ProductionQuality::Prosumer),
            3 => Ok(ProductionQuality::UserGenerated),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid ProductionQuality value: {}",
                value
            ))),
        }
    }
}