//! Non-linear creative for overlay ads

use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Container for non-linear ad creatives
///
/// Non-linear ads are overlay ads that display on top of video content
/// without interrupting playback.
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "NonLinearAds")]
pub struct NonLinearAds {
    /// Tracking events for non-linear ads
    #[xml(child = "TrackingEvents")]
    pub tracking_events: Option<TrackingEvents>,

    /// One or more non-linear ad elements
    #[xml(child = "NonLinear")]
    pub non_linear: Vec<NonLinear>,
}

/// A single non-linear ad creative
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "NonLinear")]
pub struct NonLinear {
    /// Optional identifier
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// Pixel width of the creative
    #[xml(attr = "width")]
    pub width: u32,

    /// Pixel height of the creative
    #[xml(attr = "height")]
    pub height: u32,

    /// Pixel width of the creative in its expanded state
    #[xml(attr = "expandedWidth")]
    pub expanded_width: Option<u32>,

    /// Pixel height of the creative in its expanded state
    #[xml(attr = "expandedHeight")]
    pub expanded_height: Option<u32>,

    /// Whether the creative scales with the viewport
    #[xml(attr = "scalable")]
    pub scalable: Option<bool>,

    /// Whether the creative maintains its aspect ratio when scaled
    #[xml(attr = "maintainAspectRatio")]
    pub maintain_aspect_ratio: Option<bool>,

    /// Minimum suggested duration for display
    #[xml(attr = "minSuggestedDuration")]
    pub min_suggested_duration: Option<Duration>,

    /// API framework used by the creative
    #[xml(attr = "apiFramework")]
    pub api_framework: Option<ApiFramework>,

    /// Static resource
    #[xml(child = "StaticResource")]
    pub static_resource: Option<StaticResource>,

    /// IFrame resource
    #[xml(child = "IFrameResource")]
    pub iframe_resource: Option<IFrameResource>,

    /// HTML resource
    #[xml(child = "HTMLResource")]
    pub html_resource: Option<HTMLResource>,

    /// Click handling
    #[xml(child = "NonLinearClickThrough")]
    pub click_through: Option<NonLinearClickThrough>,

    /// Click tracking
    #[xml(child = "NonLinearClickTracking")]
    pub click_tracking: Vec<NonLinearClickTracking>,

    /// Parameters for any API framework
    #[xml(child = "AdParameters")]
    pub ad_parameters: Option<AdParameters>,
}

/// Resource types for non-linear creatives
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
pub enum NonLinearResource {
    /// Static resource (image)
    #[xml(tag = "StaticResource")]
    StaticResource(StaticResource),

    /// IFrame resource
    #[xml(tag = "IFrameResource")]
    IFrameResource(IFrameResource),

    /// HTML resource
    #[xml(tag = "HTMLResource")]
    HTMLResource(HTMLResource),
}

/// Static image resource
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "StaticResource")]
pub struct StaticResource {
    /// MIME type of the resource
    #[xml(attr = "creativeType")]
    pub creative_type: String,

    /// URI to the resource
    #[xml(text)]
    pub uri: Uri,
}

/// IFrame resource
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "IFrameResource")]
pub struct IFrameResource {
    /// URI to the IFrame content
    #[xml(text)]
    pub uri: Uri,
}

/// HTML resource
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "HTMLResource")]
pub struct HTMLResource {
    /// HTML content
    #[xml(text)]
    pub html: String,
}

/// Click-through for non-linear ads
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "NonLinearClickThrough")]
pub struct NonLinearClickThrough {
    /// Click-through URI
    #[xml(text)]
    pub uri: Uri,
}

/// Click tracking for non-linear ads
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "NonLinearClickTracking")]
pub struct NonLinearClickTracking {
    /// Optional identifier
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// Click tracking URI
    #[xml(text)]
    pub uri: Uri,
}
