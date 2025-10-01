//! Ad verification for viewability and measurement

use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Container for ad verification scripts
///
/// Ad verifications allow third-party verification vendors to provide
/// executable code that measures viewability and other metrics.
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "AdVerifications")]
pub struct AdVerifications {
    /// List of verification scripts
    #[xml(child = "Verification")]
    pub verification: Vec<Verification>,
}

/// A single verification script
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Verification")]
pub struct Verification {
    /// Name of the verification vendor
    #[xml(attr = "vendor")]
    pub vendor: String,

    /// JavaScript resources for verification
    #[xml(child = "JavaScriptResource")]
    pub javascript_resources: Vec<JavaScriptResource>,

    /// Executable resources for verification
    #[xml(child = "ExecutableResource")]
    pub executable_resources: Vec<ExecutableResource>,

    /// Tracking events for verification
    #[xml(child = "TrackingEvents")]
    pub tracking_events: Option<VerificationTrackingEvents>,

    /// Parameters passed to the verification script
    #[xml(child = "VerificationParameters")]
    pub verification_parameters: Option<VerificationParameters>,
}

/// JavaScript verification resource
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "JavaScriptResource")]
pub struct JavaScriptResource {
    /// API framework (e.g., "omid")
    #[xml(attr = "apiFramework")]
    pub api_framework: ApiFramework,

    /// Whether the resource should load in a browser context
    #[xml(attr = "browserOptional")]
    pub browser_optional: Option<bool>,

    /// URI to the JavaScript resource
    #[xml(text)]
    pub uri: Uri,
}

/// Executable verification resource
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "ExecutableResource")]
pub struct ExecutableResource {
    /// API framework
    #[xml(attr = "apiFramework")]
    pub api_framework: ApiFramework,

    /// MIME type of the executable
    #[xml(attr = "type")]
    pub mime_type: Option<String>,

    /// URI to the executable resource
    #[xml(text)]
    pub uri: Uri,
}

/// Tracking events for verification
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "TrackingEvents")]
pub struct VerificationTrackingEvents {
    /// List of tracking events
    #[xml(child = "Tracking")]
    pub tracking: Vec<VerificationTracking>,
}

/// A verification tracking event
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Tracking")]
pub struct VerificationTracking {
    /// The event type (e.g., "verificationNotExecuted")
    #[xml(attr = "event")]
    pub event: String,

    /// The tracking URI
    #[xml(text)]
    pub uri: Uri,
}

/// Parameters for the verification script
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "VerificationParameters")]
pub struct VerificationParameters {
    /// The verification parameters (often Base64 encoded)
    #[xml(text)]
    pub parameters: String,
}
