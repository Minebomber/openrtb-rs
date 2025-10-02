//! Error types for VAST parsing and validation

use thiserror::Error;

/// Errors that can occur when working with VAST documents
#[derive(Debug, Error)]
pub enum VastError {
    /// XML parsing error
    #[error("XML parsing error: {0}")]
    XmlParse(#[from] quick_xml::DeError),

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

    /// Invalid tracking event
    #[error("Invalid tracking event: {0}")]
    InvalidTrackingEvent(String),

    /// Invalid companion ads required value
    #[error("Invalid companion ads required value: {0}")]
    InvalidCompanionAdsRequired(String),

    /// Invalid delivery type
    #[error("Invalid delivery type: {0}")]
    InvalidDeliveryType(String),

    /// Invalid icon position
    #[error("Invalid icon position: {0}")]
    InvalidIconPosition(String),

    /// Invalid pricing model
    #[error("Invalid pricing model: {0}")]
    InvalidPricingModel(String),
}
