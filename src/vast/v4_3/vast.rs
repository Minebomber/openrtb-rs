//! The root VAST element and its attributes

use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// The root element of a VAST document.
///
/// VAST (Video Ad Serving Template) is an XML schema that defines the structure for serving
/// video ads. The VAST element is the root of the document and contains one or more Ad elements.
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "VAST")]
pub struct Vast {
    /// The VAST version - should be "4.3" for this implementation
    #[xml(attr = "version")]
    pub version: String,

    /// Optional identifier for the VAST response
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// Indicates the sequence number of the VAST response when used in a pod
    #[xml(attr = "sequence")]
    pub sequence: Option<u32>,

    /// Indicates whether the ad is conditional (e.g., based on user opt-in)
    #[xml(attr = "conditionalAd")]
    pub conditional_ad: Option<bool>,

    /// Container for one or more Ad elements
    #[xml(child = "Ad")]
    pub ads: Vec<Ad>,

    /// URI to request if there are errors processing the VAST response
    #[xml(child = "Error")]
    pub errors: Vec<Error>,
}

impl Vast {
    /// Create a new VAST 4.3 document
    pub fn new() -> Self {
        Vast {
            version: "4.3".to_string(),
            id: None,
            sequence: None,
            conditional_ad: None,
            ads: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Add an Ad to the VAST document
    pub fn add_ad(&mut self, ad: Ad) {
        self.ads.push(ad);
    }

    /// Add an error tracking URI
    pub fn add_error(&mut self, error: Error) {
        self.errors.push(error);
    }
}

impl Default for Vast {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Vast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match hard_xml::XmlWrite::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

impl std::str::FromStr for Vast {
    type Err = hard_xml::XmlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hard_xml::XmlRead::from_str(s)
    }
}

/// Error tracking element for VAST-level errors
///
/// The Error element contains a URI that is pinged when there is an error
/// processing the VAST response.
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Error")]
pub struct Error {
    /// The error tracking URI
    #[xml(text)]
    pub uri: Uri,
}
