//! Icon elements for ad indicators

use super::enums::*;
use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Container for icon elements
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Icons")]
pub struct Icons {
    /// List of icons
    #[xml(child = "Icon")]
    pub icon: Vec<Icon>,
}

/// An icon that displays during ad playback
///
/// Icons are overlays that provide additional information about the ad,
/// such as "Ad" indicators or advertiser logos.
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Icon")]
pub struct Icon {
    /// Identifies the industry program
    #[xml(attr = "program")]
    pub program: Option<String>,

    /// Pixel width
    #[xml(attr = "width")]
    pub width: u32,

    /// Pixel height
    #[xml(attr = "height")]
    pub height: u32,

    /// X coordinate position
    #[xml(attr = "xPosition")]
    pub x_position: IconPosition,

    /// Y coordinate position
    #[xml(attr = "yPosition")]
    pub y_position: IconPosition,

    /// Duration for which the icon is displayed
    #[xml(attr = "duration")]
    pub duration: Option<Duration>,

    /// Offset time to display the icon
    #[xml(attr = "offset")]
    pub offset: Option<Duration>,

    /// API framework if applicable
    #[xml(attr = "apiFramework")]
    pub api_framework: Option<ApiFramework>,

    /// The pixel ratio for which the icon is intended
    #[xml(attr = "pxratio")]
    pub pxratio: Option<String>,

    /// Static resource
    #[xml(child = "StaticResource")]
    pub static_resource: Option<IconStaticResource>,

    /// IFrame resource
    #[xml(child = "IFrameResource")]
    pub iframe_resource: Option<IconIFrameResource>,

    /// HTML resource
    #[xml(child = "HTMLResource")]
    pub html_resource: Option<IconHTMLResource>,

    /// Icon clicks
    #[xml(child = "IconClicks")]
    pub icon_clicks: Option<IconClicks>,

    /// Icon view tracking
    #[xml(child = "IconViewTracking")]
    pub icon_view_tracking: Vec<IconViewTracking>,
}

/// Static image resource for icon
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "StaticResource")]
pub struct IconStaticResource {
    /// MIME type of the resource
    #[xml(attr = "creativeType")]
    pub creative_type: String,

    /// URI to the resource
    #[xml(text)]
    pub uri: Uri,
}

/// IFrame resource for icon
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "IFrameResource")]
pub struct IconIFrameResource {
    /// URI to the IFrame content
    #[xml(text)]
    pub uri: Uri,
}

/// HTML resource for icon
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "HTMLResource")]
pub struct IconHTMLResource {
    /// HTML content
    #[xml(text)]
    pub html: String,
}

/// Icon click handling
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "IconClicks")]
pub struct IconClicks {
    /// Click-through URL
    #[xml(child = "IconClickThrough")]
    pub click_through: Option<IconClickThrough>,

    /// Click tracking URLs
    #[xml(child = "IconClickTracking")]
    pub click_tracking: Vec<IconClickTracking>,

    /// Fallback images
    #[xml(child = "IconClickFallbackImages")]
    pub fallback_images: Option<IconClickFallbackImages>,
}

/// Icon click-through
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "IconClickThrough")]
pub struct IconClickThrough {
    /// Click-through URI
    #[xml(text)]
    pub uri: Uri,
}

/// Icon click tracking
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "IconClickTracking")]
pub struct IconClickTracking {
    /// Optional identifier
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// Click tracking URI
    #[xml(text)]
    pub uri: Uri,
}

/// Icon click fallback images
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "IconClickFallbackImages")]
pub struct IconClickFallbackImages {
    /// Fallback images
    #[xml(child = "IconClickFallbackImage")]
    pub fallback_image: Vec<IconClickFallbackImage>,
}

/// A single fallback image
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "IconClickFallbackImage")]
pub struct IconClickFallbackImage {
    /// Width of the image
    #[xml(attr = "width")]
    pub width: Option<u32>,

    /// Height of the image
    #[xml(attr = "height")]
    pub height: Option<u32>,

    /// Static resource for the fallback
    #[xml(child = "StaticResource")]
    pub static_resource: Option<IconStaticResource>,
}

/// Icon view tracking
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "IconViewTracking")]
pub struct IconViewTracking {
    /// View tracking URI
    #[xml(text)]
    pub uri: Uri,
}
