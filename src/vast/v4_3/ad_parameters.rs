//! Ad parameters for API frameworks

use hard_xml::{XmlRead, XmlWrite};

/// Parameters for any embedded API
///
/// The AdParameters element provides a string of data that can be passed
/// to the creative for use with an API framework.
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "AdParameters")]
pub struct AdParameters {
    /// Whether the parameters are XML-encoded
    #[xml(attr = "xmlEncoded")]
    pub xml_encoded: Option<bool>,

    /// The parameter data
    #[xml(text)]
    pub data: String,
}

impl AdParameters {
    /// Create new ad parameters
    pub fn new(data: String) -> Self {
        AdParameters {
            xml_encoded: None,
            data,
        }
    }

    /// Create new XML-encoded ad parameters
    pub fn new_xml_encoded(data: String) -> Self {
        AdParameters {
            xml_encoded: Some(true),
            data,
        }
    }
}
