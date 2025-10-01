use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object describes the publisher of the media in which the ad will be displayed.
///
/// The publisher is typically the seller in an OpenRTB transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Publisher<Ext = Value> {
    /// Exchange-specific publisher ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Publisher name (may be aliased at the publisher's request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Array of IAB content categories that describe the publisher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Highest level domain of the publisher (e.g., "publisher.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
