use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Further identification of a software component.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Brand<Ext = Value> {
    /// Browser name or other software component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    /// Array of version components for this software.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Vec<String>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
