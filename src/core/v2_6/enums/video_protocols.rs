use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Video Bid Response Protocols
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VideoBidResponseProtocol {
    /// VAST 1.0
    Vast1,
    /// VAST 2.0
    Vast2,
    /// VAST 3.0
    Vast3,
    /// VAST 1.0 Wrapper
    Vast1Wrapper,
    /// VAST 2.0 Wrapper
    Vast2Wrapper,
    /// VAST 3.0 Wrapper
    Vast3Wrapper,
    /// VAST 4.0
    Vast4,
    /// VAST 4.0 Wrapper
    Vast4Wrapper,
    /// DAAST 1.0
    Daast1,
    /// DAAST 1.0 Wrapper
    Daast1Wrapper,
}

impl Serialize for VideoBidResponseProtocol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            VideoBidResponseProtocol::Vast1 => serializer.serialize_u32(1),
            VideoBidResponseProtocol::Vast2 => serializer.serialize_u32(2),
            VideoBidResponseProtocol::Vast3 => serializer.serialize_u32(3),
            VideoBidResponseProtocol::Vast1Wrapper => serializer.serialize_u32(4),
            VideoBidResponseProtocol::Vast2Wrapper => serializer.serialize_u32(5),
            VideoBidResponseProtocol::Vast3Wrapper => serializer.serialize_u32(6),
            VideoBidResponseProtocol::Vast4 => serializer.serialize_u32(7),
            VideoBidResponseProtocol::Vast4Wrapper => serializer.serialize_u32(8),
            VideoBidResponseProtocol::Daast1 => serializer.serialize_u32(9),
            VideoBidResponseProtocol::Daast1Wrapper => serializer.serialize_u32(10),
        }
    }
}

impl<'de> Deserialize<'de> for VideoBidResponseProtocol {
    fn deserialize<D>(deserializer: D) -> Result<VideoBidResponseProtocol, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(VideoBidResponseProtocol::Vast1),
            2 => Ok(VideoBidResponseProtocol::Vast2),
            3 => Ok(VideoBidResponseProtocol::Vast3),
            4 => Ok(VideoBidResponseProtocol::Vast1Wrapper),
            5 => Ok(VideoBidResponseProtocol::Vast2Wrapper),
            6 => Ok(VideoBidResponseProtocol::Vast3Wrapper),
            7 => Ok(VideoBidResponseProtocol::Vast4),
            8 => Ok(VideoBidResponseProtocol::Vast4Wrapper),
            9 => Ok(VideoBidResponseProtocol::Daast1),
            10 => Ok(VideoBidResponseProtocol::Daast1Wrapper),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid VideoBidResponseProtocol value: {}",
                value
            ))),
        }
    }
}