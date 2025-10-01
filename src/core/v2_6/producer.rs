use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object defines the producer of the content in which the ad will be shown.
///
/// This is particularly useful when the content is syndicated and may be distributed through
/// different publishers and thus when the producer and publisher are not necessarily the same entity.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Producer<Ext = Value> {
    /// Content producer or originator ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Content producer or originator name (e.g., "Warner Bros").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Array of IAB content categories that describe the content producer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Highest level domain of the content producer (e.g., "producer.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
