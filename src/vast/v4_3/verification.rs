//! Ad verification for viewability and measurement

use super::*;
use serde::{Deserialize, Serialize};

/// Container for ad verification scripts
///
/// Ad verifications allow third-party verification vendors to provide
/// executable code that measures viewability and other metrics.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "AdVerifications")]
pub struct AdVerifications {
    /// List of verification scripts
    #[serde(
        rename = "Verification",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub verification: Vec<Verification>,
}

/// A single verification script
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Verification")]
pub struct Verification {
    /// Name of the verification vendor
    #[serde(rename = "@vendor")]
    pub vendor: String,

    /// JavaScript resources for verification
    #[serde(
        rename = "JavaScriptResource",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub javascript_resources: Vec<JavaScriptResource>,

    /// Executable resources for verification
    #[serde(
        rename = "ExecutableResource",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub executable_resources: Vec<ExecutableResource>,

    /// Tracking events for verification
    #[serde(rename = "TrackingEvents", skip_serializing_if = "Option::is_none")]
    pub tracking_events: Option<VerificationTrackingEvents>,

    /// Parameters passed to the verification script
    #[serde(
        rename = "VerificationParameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_parameters: Option<VerificationParameters>,
}

/// JavaScript verification resource
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "JavaScriptResource")]
pub struct JavaScriptResource {
    /// API framework (e.g., "omid")
    #[serde(rename = "@apiFramework")]
    pub api_framework: ApiFramework,

    /// Whether the resource should load in a browser context
    #[serde(rename = "@browserOptional", skip_serializing_if = "Option::is_none")]
    pub browser_optional: Option<bool>,

    /// URI to the JavaScript resource
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Executable verification resource
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ExecutableResource")]
pub struct ExecutableResource {
    /// API framework
    #[serde(rename = "@apiFramework")]
    pub api_framework: ApiFramework,

    /// MIME type of the executable
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// URI to the executable resource
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Tracking events for verification
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "TrackingEvents")]
pub struct VerificationTrackingEvents {
    /// List of tracking events
    #[serde(rename = "Tracking", default, skip_serializing_if = "Vec::is_empty")]
    pub tracking: Vec<VerificationTracking>,
}

/// A verification tracking event
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Tracking")]
pub struct VerificationTracking {
    /// The event type (e.g., "verificationNotExecuted")
    #[serde(rename = "@event")]
    pub event: String,

    /// The tracking URI
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Parameters for the verification script
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "VerificationParameters")]
pub struct VerificationParameters {
    /// The verification parameters (often Base64 encoded)
    #[serde(rename = "$value")]
    pub parameters: String,
}
