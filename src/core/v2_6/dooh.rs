use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object should be included if the ad supported content is a Digital Out-Of-Home (DOOH) screen.
///
/// A bid request with a DOOH object must not contain a site or app object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dooh {
    /// Exchange-specific DOOH ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// DOOH name (may be aliased at the publisher's request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of out-of-home venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venuetype: Option<Vec<String>>,

    /// The type of venue taxonomy the venue is defined as.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venuetax: Option<u32>,

    /// Details about the Publisher of the DOOH medium.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,

    /// Domain of the inventory source (e.g., "doohprovider.foo.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Array of content categories describing the DOOH medium using IDs from the taxonomy indicated in cattax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// The taxonomy in use for the cat attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cattax: Option<u32>,

    /// Comma separated list of keywords about the DOOH medium.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Details about the Content associated with the DOOH medium.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
