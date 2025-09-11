use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object represents a native type impression.
///
/// Native ad units are intended to blend seamlessly into the surrounding content. The Native Subcommittee
/// has developed a companion specification to OpenRTB called the Dynamic Native Ads API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Native {
    /// Request payload complying with the Native Ad Specification.
    pub request: String,

    /// Version of the Dynamic Native Ads API to which request complies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// List of supported API frameworks for this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Blocked creative attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i32>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
