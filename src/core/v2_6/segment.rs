use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Segment objects are essentially key-value pairs that convey specific units of data.
///
/// The parent Data object is a collection of such values from a given data provider.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Segment<Ext = Value> {
    /// ID of the data segment specific to the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of the data segment specific to the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// String representation of the data segment value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
