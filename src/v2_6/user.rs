use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object contains information known or derived about the human user of the device.
///
/// The user id is an exchange artifact and may be subject to rotation policies. On mobile, this may
/// correspond to the ID for Advertisers (IFA).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    /// Exchange-specific ID for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Buyer-specific ID for the user as mapped by the exchange for the buyer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyeruid: Option<String>,

    /// Year of birth as a 4-digit integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yob: Option<i32>,

    /// Gender, where "M" = male, "F" = female, "O" = known to be other.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    /// Comma separated list of keywords, interests, or intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Optional feature to pass bidder data that was set in the exchange's cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customdata: Option<String>,

    /// Location of the user's home base defined by a `Geo` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,

    /// Additional user data. Each `Data` object represents a different data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Data>>,

    /// Structured user agent information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sua: Option<UserAgent>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
