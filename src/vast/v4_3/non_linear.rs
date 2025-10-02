//! Non-linear creative for overlay ads

use super::*;
use serde::{Deserialize, Serialize};

/// Container for non-linear ad creatives
///
/// Non-linear ads are overlay ads that display on top of video content
/// without interrupting playback.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "NonLinearAds")]
pub struct NonLinearAds {
    /// Tracking events for non-linear ads
    #[serde(rename = "TrackingEvents", skip_serializing_if = "Option::is_none")]
    pub tracking_events: Option<TrackingEvents>,

    /// One or more non-linear ad elements
    #[serde(rename = "NonLinear", default, skip_serializing_if = "Vec::is_empty")]
    pub non_linear: Vec<NonLinear>,
}

/// A single non-linear ad creative
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "NonLinear")]
pub struct NonLinear {
    /// Optional identifier
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Pixel width of the creative
    #[serde(rename = "@width")]
    pub width: u32,

    /// Pixel height of the creative
    #[serde(rename = "@height")]
    pub height: u32,

    /// Pixel width of the creative in its expanded state
    #[serde(rename = "@expandedWidth", skip_serializing_if = "Option::is_none")]
    pub expanded_width: Option<u32>,

    /// Pixel height of the creative in its expanded state
    #[serde(rename = "@expandedHeight", skip_serializing_if = "Option::is_none")]
    pub expanded_height: Option<u32>,

    /// Whether the creative scales with the viewport
    #[serde(rename = "@scalable", skip_serializing_if = "Option::is_none")]
    pub scalable: Option<bool>,

    /// Whether the creative maintains its aspect ratio when scaled
    #[serde(
        rename = "@maintainAspectRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintain_aspect_ratio: Option<bool>,

    /// Minimum suggested duration for display
    #[serde(
        rename = "@minSuggestedDuration",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_suggested_duration: Option<Duration>,

    /// API framework used by the creative
    #[serde(rename = "@apiFramework", skip_serializing_if = "Option::is_none")]
    pub api_framework: Option<ApiFramework>,

    /// Static resource
    #[serde(rename = "StaticResource", skip_serializing_if = "Option::is_none")]
    pub static_resource: Option<StaticResource>,

    /// IFrame resource
    #[serde(rename = "IFrameResource", skip_serializing_if = "Option::is_none")]
    pub iframe_resource: Option<IFrameResource>,

    /// HTML resource
    #[serde(rename = "HTMLResource", skip_serializing_if = "Option::is_none")]
    pub html_resource: Option<HTMLResource>,

    /// Click handling
    #[serde(
        rename = "NonLinearClickThrough",
        skip_serializing_if = "Option::is_none"
    )]
    pub click_through: Option<NonLinearClickThrough>,

    /// Click tracking
    #[serde(
        rename = "NonLinearClickTracking",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub click_tracking: Vec<NonLinearClickTracking>,

    /// Parameters for any API framework
    #[serde(rename = "AdParameters", skip_serializing_if = "Option::is_none")]
    pub ad_parameters: Option<AdParameters>,
}

/// Static image resource
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "StaticResource")]
pub struct StaticResource {
    /// MIME type of the resource
    #[serde(rename = "@creativeType")]
    pub creative_type: String,

    /// URI to the resource
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// IFrame resource
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "IFrameResource")]
pub struct IFrameResource {
    /// URI to the IFrame content
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// HTML resource
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "HTMLResource")]
pub struct HTMLResource {
    /// HTML content
    #[serde(rename = "$value")]
    pub html: String,
}

/// Click-through for non-linear ads
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "NonLinearClickThrough")]
pub struct NonLinearClickThrough {
    /// Click-through URI
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Click tracking for non-linear ads
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "NonLinearClickTracking")]
pub struct NonLinearClickTracking {
    /// Optional identifier
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Click tracking URI
    #[serde(rename = "$value")]
    pub uri: Uri,
}
