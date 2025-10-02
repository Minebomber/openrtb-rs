//! Icon elements for ad indicators

use super::*;
use serde::{Deserialize, Serialize};

/// Icon position (can be pixel value or left/right/top/bottom)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IconPosition {
    /// Left position
    #[serde(rename = "left")]
    Left,
    /// Right position
    #[serde(rename = "right")]
    Right,
    /// Top position
    #[serde(rename = "top")]
    Top,
    /// Bottom position
    #[serde(rename = "bottom")]
    Bottom,
    /// Numeric pixel position
    Pixels(u32),
}

impl std::fmt::Display for IconPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IconPosition::Left => write!(f, "left"),
            IconPosition::Right => write!(f, "right"),
            IconPosition::Top => write!(f, "top"),
            IconPosition::Bottom => write!(f, "bottom"),
            IconPosition::Pixels(n) => write!(f, "{}", n),
        }
    }
}

impl std::str::FromStr for IconPosition {
    type Err = VastError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "left" => IconPosition::Left,
            "right" => IconPosition::Right,
            "top" => IconPosition::Top,
            "bottom" => IconPosition::Bottom,
            _ => s
                .parse::<u32>()
                .map(IconPosition::Pixels)
                .map_err(|_| VastError::InvalidIconPosition(s.to_string()))?,
        })
    }
}

/// Container for icon elements
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Icons")]
pub struct Icons {
    /// List of icons
    #[serde(rename = "Icon", default, skip_serializing_if = "Vec::is_empty")]
    pub icon: Vec<Icon>,
}

/// An icon that displays during ad playback
///
/// Icons are overlays that provide additional information about the ad,
/// such as "Ad" indicators or advertiser logos.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Icon")]
pub struct Icon {
    /// Identifies the industry program
    #[serde(rename = "@program", skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,

    /// Pixel width
    #[serde(rename = "@width")]
    pub width: u32,

    /// Pixel height
    #[serde(rename = "@height")]
    pub height: u32,

    /// X coordinate position
    #[serde(rename = "@xPosition")]
    pub x_position: IconPosition,

    /// Y coordinate position
    #[serde(rename = "@yPosition")]
    pub y_position: IconPosition,

    /// Duration for which the icon is displayed
    #[serde(rename = "@duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,

    /// Offset time to display the icon
    #[serde(rename = "@offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<Duration>,

    /// API framework if applicable
    #[serde(rename = "@apiFramework", skip_serializing_if = "Option::is_none")]
    pub api_framework: Option<ApiFramework>,

    /// The pixel ratio for which the icon is intended
    #[serde(rename = "@pxratio", skip_serializing_if = "Option::is_none")]
    pub pxratio: Option<String>,

    /// Static resource
    #[serde(rename = "StaticResource", skip_serializing_if = "Option::is_none")]
    pub static_resource: Option<IconStaticResource>,

    /// IFrame resource
    #[serde(rename = "IFrameResource", skip_serializing_if = "Option::is_none")]
    pub iframe_resource: Option<IconIFrameResource>,

    /// HTML resource
    #[serde(rename = "HTMLResource", skip_serializing_if = "Option::is_none")]
    pub html_resource: Option<IconHTMLResource>,

    /// Icon clicks
    #[serde(rename = "IconClicks", skip_serializing_if = "Option::is_none")]
    pub icon_clicks: Option<IconClicks>,

    /// Icon view tracking
    #[serde(
        rename = "IconViewTracking",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub icon_view_tracking: Vec<IconViewTracking>,
}

/// Static image resource for icon
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "StaticResource")]
pub struct IconStaticResource {
    /// MIME type of the resource
    #[serde(rename = "@creativeType")]
    pub creative_type: String,

    /// URI to the resource
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// IFrame resource for icon
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "IFrameResource")]
pub struct IconIFrameResource {
    /// URI to the IFrame content
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// HTML resource for icon
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "HTMLResource")]
pub struct IconHTMLResource {
    /// HTML content
    #[serde(rename = "$value")]
    pub html: String,
}

/// Icon click handling
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "IconClicks")]
pub struct IconClicks {
    /// Click-through URL
    #[serde(rename = "IconClickThrough", skip_serializing_if = "Option::is_none")]
    pub click_through: Option<IconClickThrough>,

    /// Click tracking URLs
    #[serde(
        rename = "IconClickTracking",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub click_tracking: Vec<IconClickTracking>,

    /// Fallback images
    #[serde(
        rename = "IconClickFallbackImages",
        skip_serializing_if = "Option::is_none"
    )]
    pub fallback_images: Option<IconClickFallbackImages>,
}

/// Icon click-through
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "IconClickThrough")]
pub struct IconClickThrough {
    /// Click-through URI
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Icon click tracking
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "IconClickTracking")]
pub struct IconClickTracking {
    /// Optional identifier
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Click tracking URI
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Icon click fallback images
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "IconClickFallbackImages")]
pub struct IconClickFallbackImages {
    /// Fallback images
    #[serde(
        rename = "IconClickFallbackImage",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fallback_image: Vec<IconClickFallbackImage>,
}

/// A single fallback image
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "IconClickFallbackImage")]
pub struct IconClickFallbackImage {
    /// Width of the image
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// Height of the image
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    /// Static resource for the fallback
    #[serde(rename = "StaticResource", skip_serializing_if = "Option::is_none")]
    pub static_resource: Option<IconStaticResource>,
}

/// Icon view tracking
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "IconViewTracking")]
pub struct IconViewTracking {
    /// View tracking URI
    #[serde(rename = "$value")]
    pub uri: Uri,
}
