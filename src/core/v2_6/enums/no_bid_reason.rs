use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// No-Bid Reason Codes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoBidReasonCode {
    /// Unknown Error
    UnknownError,
    /// Technical Error
    TechnicalError,
    /// Invalid Request
    InvalidRequest,
    /// Known Web Spider
    KnownWebSpider,
    /// Suspected Non-Human Traffic
    SuspectedNonHumanTraffic,
    /// Cloud, Data center, or Proxy Traffic
    CloudDataCenterProxyTraffic,
    /// Unsupported Device
    UnsupportedDevice,
    /// Blocked Publisher or Site
    BlockedPublisherOrSite,
    /// Unmatched User
    UnmatchedUser,
    /// Daily Reader Cap Met
    DailyReaderCapMet,
    /// Daily Domain Cap Met
    DailyDomainCapMet,
}

impl Serialize for NoBidReasonCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            NoBidReasonCode::UnknownError => serializer.serialize_u32(0),
            NoBidReasonCode::TechnicalError => serializer.serialize_u32(1),
            NoBidReasonCode::InvalidRequest => serializer.serialize_u32(2),
            NoBidReasonCode::KnownWebSpider => serializer.serialize_u32(3),
            NoBidReasonCode::SuspectedNonHumanTraffic => serializer.serialize_u32(4),
            NoBidReasonCode::CloudDataCenterProxyTraffic => serializer.serialize_u32(5),
            NoBidReasonCode::UnsupportedDevice => serializer.serialize_u32(6),
            NoBidReasonCode::BlockedPublisherOrSite => serializer.serialize_u32(7),
            NoBidReasonCode::UnmatchedUser => serializer.serialize_u32(8),
            NoBidReasonCode::DailyReaderCapMet => serializer.serialize_u32(9),
            NoBidReasonCode::DailyDomainCapMet => serializer.serialize_u32(10),
        }
    }
}

impl<'de> Deserialize<'de> for NoBidReasonCode {
    fn deserialize<D>(deserializer: D) -> Result<NoBidReasonCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(NoBidReasonCode::UnknownError),
            1 => Ok(NoBidReasonCode::TechnicalError),
            2 => Ok(NoBidReasonCode::InvalidRequest),
            3 => Ok(NoBidReasonCode::KnownWebSpider),
            4 => Ok(NoBidReasonCode::SuspectedNonHumanTraffic),
            5 => Ok(NoBidReasonCode::CloudDataCenterProxyTraffic),
            6 => Ok(NoBidReasonCode::UnsupportedDevice),
            7 => Ok(NoBidReasonCode::BlockedPublisherOrSite),
            8 => Ok(NoBidReasonCode::UnmatchedUser),
            9 => Ok(NoBidReasonCode::DailyReaderCapMet),
            10 => Ok(NoBidReasonCode::DailyDomainCapMet),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid NoBidReasonCode value: {}",
                value
            ))),
        }
    }
}