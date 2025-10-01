//! Error types for VAST parsing and validation

use thiserror::Error;

/// Errors that can occur when working with VAST documents
#[derive(Debug, Error)]
pub enum VastError {
    /// XML parsing error
    #[error("XML parsing error: {0}")]
    XmlParse(#[from] hard_xml::XmlError),

    /// Invalid VAST version
    #[error("Invalid VAST version: expected 4.3, got {0}")]
    InvalidVersion(String),

    /// Required element is missing
    #[error("Required element '{0}' is missing")]
    MissingElement(String),

    /// Invalid attribute value
    #[error("Invalid attribute value for '{0}': {1}")]
    InvalidAttribute(String, String),

    /// URL parsing error
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// Duration parsing error
    #[error("Invalid duration format: {0}")]
    InvalidDuration(String),
}

