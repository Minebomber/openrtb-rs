//! Universal Ad ID for creative identification

use serde::{Deserialize, Serialize};

/// Universal Ad ID element
///
/// The UniversalAdId element provides a unique creative identifier
/// that is maintained across systems.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "UniversalAdId")]
pub struct UniversalAdId {
    /// The registry that issued the ID
    #[serde(rename = "@idRegistry")]
    pub id_registry: String,

    /// Optional value from the registry
    #[serde(rename = "@idValue", skip_serializing_if = "Option::is_none")]
    pub id_value: Option<String>,

    /// The universal ad ID value
    #[serde(rename = "$value")]
    pub value: String,
}

impl UniversalAdId {
    /// Create a new UniversalAdId
    pub fn new(id_registry: String, value: String) -> Self {
        UniversalAdId {
            id_registry,
            id_value: None,
            value,
        }
    }
}
