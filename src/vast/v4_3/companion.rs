//! Companion banner ads that display alongside video content

use super::enums::*;
use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Container for companion ad creatives
///
/// Companion ads are banner-style ads that appear around the video player
/// and remain visible while video content plays.
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "CompanionAds")]
pub struct CompanionAds {
    /// Indicates whether companion ads are required
    #[xml(attr = "required")]
    pub required: Option<CompanionAdsRequired>,

    /// One or more companion ad elements
    #[xml(child = "Companion")]
    pub companion: Vec<CompanionAd>,
}

/// A single companion ad
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Companion")]
pub struct CompanionAd {
    /// Optional identifier
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// Pixel width of the companion
    #[xml(attr = "width")]
    pub width: u32,

    /// Pixel height of the companion
    #[xml(attr = "height")]
    pub height: u32,

    /// Identifier for the creative asset
    #[xml(attr = "assetWidth")]
    pub asset_width: Option<u32>,

    /// Identifier for the creative asset
    #[xml(attr = "assetHeight")]
    pub asset_height: Option<u32>,

    /// Pixel width of the companion in its expanded state
    #[xml(attr = "expandedWidth")]
    pub expanded_width: Option<u32>,

    /// Pixel height of the companion in its expanded state
    #[xml(attr = "expandedHeight")]
    pub expanded_height: Option<u32>,

    /// API framework used by the companion
    #[xml(attr = "apiFramework")]
    pub api_framework: Option<ApiFramework>,

    /// Identifies the rendering mode
    #[xml(attr = "renderingMode")]
    pub rendering_mode: Option<String>,

    /// The pixel ratio for which the companion creative is intended
    #[xml(attr = "pxratio")]
    pub pxratio: Option<String>,

    /// Static resource
    #[xml(child = "StaticResource")]
    pub static_resource: Option<CompanionStaticResource>,

    /// IFrame resource
    #[xml(child = "IFrameResource")]
    pub iframe_resource: Option<CompanionIFrameResource>,

    /// HTML resource
    #[xml(child = "HTMLResource")]
    pub html_resource: Option<CompanionHTMLResource>,

    /// Alt text for the companion
    #[xml(child = "AltText")]
    pub alt_text: Option<AltText>,

    /// Click-through URL
    #[xml(child = "CompanionClickThrough")]
    pub click_through: Option<CompanionClickThrough>,

    /// Click tracking URLs
    #[xml(child = "CompanionClickTracking")]
    pub click_tracking: Vec<CompanionClickTracking>,

    /// Tracking events
    #[xml(child = "TrackingEvents")]
    pub tracking_events: Option<TrackingEvents>,

    /// Parameters for API frameworks
    #[xml(child = "AdParameters")]
    pub ad_parameters: Option<AdParameters>,
}

/// Static image resource for companion
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "StaticResource")]
pub struct CompanionStaticResource {
    /// MIME type of the resource
    #[xml(attr = "creativeType")]
    pub creative_type: String,

    /// URI to the resource
    #[xml(text)]
    pub uri: Uri,
}

/// IFrame resource for companion
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "IFrameResource")]
pub struct CompanionIFrameResource {
    /// URI to the IFrame content
    #[xml(text)]
    pub uri: Uri,
}

/// HTML resource for companion
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "HTMLResource")]
pub struct CompanionHTMLResource {
    /// HTML content
    #[xml(text)]
    pub html: String,
}

/// Click-through for companion ads
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "CompanionClickThrough")]
pub struct CompanionClickThrough {
    /// Click-through URI
    #[xml(text)]
    pub uri: Uri,
}

/// Click tracking for companion ads
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "CompanionClickTracking")]
pub struct CompanionClickTracking {
    /// Optional identifier
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// Click tracking URI
    #[xml(text)]
    pub uri: Uri,
}

/// Alt text wrapper
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "AltText")]
pub struct AltText {
    #[xml(text)]
    pub value: String,
}
