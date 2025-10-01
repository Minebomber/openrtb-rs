use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object is used to define a content network the content belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Network<Ext = Value> {
    /// Network the content belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Network name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Network domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
