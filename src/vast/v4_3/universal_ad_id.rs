//! Universal Ad ID for creative identification

use hard_xml::{XmlRead, XmlWrite};

/// Universal Ad ID element
///
/// The UniversalAdId element provides a unique creative identifier
/// that is maintained across systems.
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "UniversalAdId")]
pub struct UniversalAdId {
    /// The registry that issued the ID
    #[xml(attr = "idRegistry")]
    pub id_registry: String,

    /// Optional value from the registry
    #[xml(attr = "idValue")]
    pub id_value: Option<String>,

    /// The universal ad ID value
    #[xml(text)]
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
