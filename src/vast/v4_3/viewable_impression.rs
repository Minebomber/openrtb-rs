//! Viewable impression tracking

use super::*;
use serde::{Deserialize, Serialize};

/// Container for viewable impression tracking
///
/// Viewable impression tracking allows measurement of whether an ad
/// was actually viewable according to industry standards.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ViewableImpression")]
pub struct ViewableImpression {
    /// Optional identifier
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Viewable event tracking
    #[serde(rename = "Viewable", default, skip_serializing_if = "Vec::is_empty")]
    pub viewable: Vec<Viewable>,

    /// Not viewable event tracking
    #[serde(rename = "NotViewable", default, skip_serializing_if = "Vec::is_empty")]
    pub not_viewable: Vec<NotViewable>,

    /// Viewability undetermined tracking
    #[serde(
        rename = "ViewUndetermined",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub view_undetermined: Vec<ViewUndetermined>,
}

/// Tracking for viewable impressions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Viewable")]
pub struct Viewable {
    /// Tracking URI for viewable event
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Tracking for non-viewable impressions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "NotViewable")]
pub struct NotViewable {
    /// Tracking URI for not viewable event
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Tracking for undetermined viewability
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ViewUndetermined")]
pub struct ViewUndetermined {
    /// Tracking URI for undetermined viewability
    #[serde(rename = "$value")]
    pub uri: Uri,
}
