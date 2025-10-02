//! Linear creative for video and audio ads

use super::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Skip offset can be percentage or time
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SkipOffset {
    /// Time offset in HH:MM:SS format
    Time(Duration),
    /// Percentage offset (e.g., "25%")
    Percentage(String),
}

impl fmt::Display for SkipOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkipOffset::Time(d) => write!(f, "{}", d),
            SkipOffset::Percentage(p) => write!(f, "{}", p),
        }
    }
}

impl std::str::FromStr for SkipOffset {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('%') {
            Ok(SkipOffset::Percentage(s.to_string()))
        } else {
            Ok(SkipOffset::Time(Duration {
                value: s.to_string(),
            }))
        }
    }
}

/// Linear creative that plays linearly with the video content.
///
/// Linear ads are video or audio ads that play before (pre-roll), during (mid-roll),
/// or after (post-roll) the video content.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Linear")]
pub struct Linear {
    /// Indicates whether the player can skip the creative
    #[serde(rename = "@skipoffset", skip_serializing_if = "Option::is_none")]
    pub skip_offset: Option<SkipOffset>,

    /// Duration of the creative in HH:MM:SS format
    #[serde(rename = "Duration")]
    pub duration: Duration,

    /// Media files for the creative
    #[serde(rename = "MediaFiles")]
    pub media_files: MediaFiles,

    /// Parameters for any embedded API
    #[serde(rename = "AdParameters", skip_serializing_if = "Option::is_none")]
    pub ad_parameters: Option<AdParameters>,

    /// Video click tracking
    #[serde(rename = "VideoClicks", skip_serializing_if = "Option::is_none")]
    pub video_clicks: Option<VideoClicks>,

    /// Tracking events
    #[serde(rename = "TrackingEvents", skip_serializing_if = "Option::is_none")]
    pub tracking_events: Option<TrackingEvents>,

    /// Icon elements
    #[serde(rename = "Icons", skip_serializing_if = "Option::is_none")]
    pub icons: Option<Icons>,
}

/// Container for video click tracking elements
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "VideoClicks")]
pub struct VideoClicks {
    /// URI to open when the viewer clicks the video
    #[serde(rename = "ClickThrough", skip_serializing_if = "Option::is_none")]
    pub click_through: Option<ClickThrough>,

    /// URIs to ping when the viewer clicks the video
    #[serde(
        rename = "ClickTracking",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub click_tracking: Vec<ClickTracking>,

    /// Custom click URIs
    #[serde(rename = "CustomClick", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_clicks: Vec<CustomClick>,
}

/// Click-through URL
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ClickThrough")]
pub struct ClickThrough {
    /// Optional identifier
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The click-through URL
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Click tracking URL
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ClickTracking")]
pub struct ClickTracking {
    /// Optional identifier
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The click tracking URL
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Custom click URL
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "CustomClick")]
pub struct CustomClick {
    /// Optional identifier
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The custom click URL
    #[serde(rename = "$value")]
    pub uri: Uri,
}
