use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Auction Type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuctionType {
    /// First Price
    FirstPrice,
    /// Second Price Plus
    SecondPricePlus,
}

impl Serialize for AuctionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            AuctionType::FirstPrice => serializer.serialize_u32(1),
            AuctionType::SecondPricePlus => serializer.serialize_u32(2),
        }
    }
}

impl<'de> Deserialize<'de> for AuctionType {
    fn deserialize<D>(deserializer: D) -> Result<AuctionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(AuctionType::FirstPrice),
            2 => Ok(AuctionType::SecondPricePlus),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid AuctionType value: {}",
                value
            ))),
        }
    }
}
