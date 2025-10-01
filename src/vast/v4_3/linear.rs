//! Linear creative for video and audio ads

use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Linear creative that plays linearly with the video content.
///
/// Linear ads are video or audio ads that play before (pre-roll), during (mid-roll),
/// or after (post-roll) the video content.
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Linear")]
pub struct Linear {
    /// Indicates whether the player can skip the creative
    #[xml(attr = "skipoffset")]
    pub skip_offset: Option<SkipOffset>,

    /// Duration of the creative in HH:MM:SS format
    #[xml(child = "Duration")]
    pub duration: Duration,

    /// Media files for the creative
    #[xml(child = "MediaFiles")]
    pub media_files: MediaFiles,

    /// Parameters for any embedded API
    #[xml(child = "AdParameters")]
    pub ad_parameters: Option<AdParameters>,

    /// Video click tracking
    #[xml(child = "VideoClicks")]
    pub video_clicks: Option<VideoClicks>,

    /// Tracking events
    #[xml(child = "TrackingEvents")]
    pub tracking_events: Option<TrackingEvents>,

    /// Icon elements
    #[xml(child = "Icons")]
    pub icons: Option<Icons>,
}

/// Container for video click tracking elements
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "VideoClicks")]
pub struct VideoClicks {
    /// URI to open when the viewer clicks the video
    #[xml(child = "ClickThrough")]
    pub click_through: Option<ClickThrough>,

    /// URIs to ping when the viewer clicks the video
    #[xml(child = "ClickTracking")]
    pub click_tracking: Vec<ClickTracking>,

    /// Custom click URIs
    #[xml(child = "CustomClick")]
    pub custom_clicks: Vec<CustomClick>,
}

/// Click-through URL
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "ClickThrough")]
pub struct ClickThrough {
    /// Optional identifier
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// The click-through URL
    #[xml(text)]
    pub uri: Uri,
}

/// Click tracking URL
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "ClickTracking")]
pub struct ClickTracking {
    /// Optional identifier
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// The click tracking URL
    #[xml(text)]
    pub uri: Uri,
}

/// Custom click URL
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "CustomClick")]
pub struct CustomClick {
    /// Optional identifier
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// The custom click URL
    #[xml(text)]
    pub uri: Uri,
}
