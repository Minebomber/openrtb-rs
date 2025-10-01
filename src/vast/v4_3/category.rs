//! Category information for ads

use hard_xml::{XmlRead, XmlWrite};

/// Category element for ad categorization
///
/// The Category element provides IAB standard content categories
/// for the ad content.
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Category")]
pub struct Category {
    /// The authority that defines the categorization scheme
    #[xml(attr = "authority")]
    pub authority: String,

    /// The category code/identifier
    #[xml(text)]
    pub code: String,
}

impl Category {
    /// Create a new category with IAB authority
    pub fn new_iab(code: String) -> Self {
        Category {
            authority: "IAB".to_string(),
            code,
        }
    }

    /// Create a new category with custom authority
    pub fn new(authority: String, code: String) -> Self {
        Category { authority, code }
    }
}
