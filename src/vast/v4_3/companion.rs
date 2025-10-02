//! Companion banner ads that display alongside video content

use super::*;
use serde::{Deserialize, Serialize};

/// Companion ad requirement options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompanionAdsRequired {
    /// All companion ads must be displayed
    #[serde(rename = "all")]
    All,
    /// At least one companion ad must be displayed
    #[serde(rename = "any")]
    Any,
    /// No requirement for companion ad display
    #[serde(rename = "none")]
    None,
}

impl std::fmt::Display for CompanionAdsRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompanionAdsRequired::All => write!(f, "all"),
            CompanionAdsRequired::Any => write!(f, "any"),
            CompanionAdsRequired::None => write!(f, "none"),
        }
    }
}

impl std::str::FromStr for CompanionAdsRequired {
    type Err = VastError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "all" => CompanionAdsRequired::All,
            "any" => CompanionAdsRequired::Any,
            "none" => CompanionAdsRequired::None,
            _ => {
                return Err(VastError::InvalidCompanionAdsRequired(s.to_string()));
            }
        })
    }
}

/// Container for companion ad creatives
///
/// Companion ads are banner-style ads that appear around the video player
/// and remain visible while video content plays.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "CompanionAds")]
pub struct CompanionAds {
    /// Indicates whether companion ads are required
    #[serde(rename = "@required", skip_serializing_if = "Option::is_none")]
    pub required: Option<CompanionAdsRequired>,

    /// One or more companion ad elements
    #[serde(rename = "Companion", default, skip_serializing_if = "Vec::is_empty")]
    pub companion: Vec<CompanionAd>,
}

/// A single companion ad
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Companion")]
pub struct CompanionAd {
    /// Optional identifier
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Pixel width of the companion
    #[serde(rename = "@width")]
    pub width: u32,

    /// Pixel height of the companion
    #[serde(rename = "@height")]
    pub height: u32,

    /// Identifier for the creative asset
    #[serde(rename = "@assetWidth", skip_serializing_if = "Option::is_none")]
    pub asset_width: Option<u32>,

    /// Identifier for the creative asset
    #[serde(rename = "@assetHeight", skip_serializing_if = "Option::is_none")]
    pub asset_height: Option<u32>,

    /// Pixel width of the companion in its expanded state
    #[serde(rename = "@expandedWidth", skip_serializing_if = "Option::is_none")]
    pub expanded_width: Option<u32>,

    /// Pixel height of the companion in its expanded state
    #[serde(rename = "@expandedHeight", skip_serializing_if = "Option::is_none")]
    pub expanded_height: Option<u32>,

    /// API framework used by the companion
    #[serde(rename = "@apiFramework", skip_serializing_if = "Option::is_none")]
    pub api_framework: Option<ApiFramework>,

    /// Identifies the rendering mode
    #[serde(rename = "@renderingMode", skip_serializing_if = "Option::is_none")]
    pub rendering_mode: Option<String>,

    /// The pixel ratio for which the companion creative is intended
    #[serde(rename = "@pxratio", skip_serializing_if = "Option::is_none")]
    pub pxratio: Option<String>,

    /// Static resource
    #[serde(rename = "StaticResource", skip_serializing_if = "Option::is_none")]
    pub static_resource: Option<CompanionStaticResource>,

    /// IFrame resource
    #[serde(rename = "IFrameResource", skip_serializing_if = "Option::is_none")]
    pub iframe_resource: Option<CompanionIFrameResource>,

    /// HTML resource
    #[serde(rename = "HTMLResource", skip_serializing_if = "Option::is_none")]
    pub html_resource: Option<CompanionHTMLResource>,

    /// Alt text for the companion
    #[serde(rename = "AltText", skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<AltText>,

    /// Click-through URL
    #[serde(
        rename = "CompanionClickThrough",
        skip_serializing_if = "Option::is_none"
    )]
    pub click_through: Option<CompanionClickThrough>,

    /// Click tracking URLs
    #[serde(
        rename = "CompanionClickTracking",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub click_tracking: Vec<CompanionClickTracking>,

    /// Tracking events
    #[serde(rename = "TrackingEvents", skip_serializing_if = "Option::is_none")]
    pub tracking_events: Option<TrackingEvents>,

    /// Parameters for API frameworks
    #[serde(rename = "AdParameters", skip_serializing_if = "Option::is_none")]
    pub ad_parameters: Option<AdParameters>,
}

/// Static image resource for companion
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "StaticResource")]
pub struct CompanionStaticResource {
    /// MIME type of the resource
    #[serde(rename = "@creativeType")]
    pub creative_type: String,

    /// URI to the resource
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// IFrame resource for companion
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "IFrameResource")]
pub struct CompanionIFrameResource {
    /// URI to the IFrame content
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// HTML resource for companion
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "HTMLResource")]
pub struct CompanionHTMLResource {
    /// HTML content
    #[serde(rename = "$value")]
    pub html: String,
}

/// Click-through for companion ads
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "CompanionClickThrough")]
pub struct CompanionClickThrough {
    /// Click-through URI
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Click tracking for companion ads
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "CompanionClickTracking")]
pub struct CompanionClickTracking {
    /// Optional identifier
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Click tracking URI
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Alt text wrapper
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "AltText")]
pub struct AltText {
    #[serde(rename = "$value")]
    pub value: String,
}
