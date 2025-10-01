use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object is used to define a channel the content belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Channel<Ext = Value> {
    /// Platform-specific channel identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Channel name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Channel domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
