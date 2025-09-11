use serde::{Deserialize, Serialize};

/// No-Bid Reason Codes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum NoBidReasonCode {
    /// Unknown Error
    UnknownError = 0,
    /// Technical Error
    TechnicalError = 1,
    /// Invalid Request
    InvalidRequest = 2,
    /// Known Web Spider
    KnownWebSpider = 3,
    /// Suspected Non-Human Traffic
    SuspectedNonHumanTraffic = 4,
    /// Cloud, Data center, or Proxy Traffic
    CloudDataCenterProxyTraffic = 5,
    /// Unsupported Device
    UnsupportedDevice = 6,
    /// Blocked Publisher or Site
    BlockedPublisherOrSite = 7,
    /// Unmatched User
    UnmatchedUser = 8,
    /// Daily Reader Cap Met
    DailyReaderCapMet = 9,
    /// Daily Domain Cap Met
    DailyDomainCapMet = 10,
}

