use super::*;
use crate::enums::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object represents the most general type of impression.
///
/// Although the term "banner" may have very specific meaning in other contexts, here it can be
/// many things including a simple static image, an expandable ad unit, or even in-banner video.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Banner {
    /// Array of format objects representing the banner sizes permitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Vec<Format>>,

    /// Exact width in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Exact height in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// NOTE: Deprecated in favor of the format array.
    /// Maximum width in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmax: Option<i32>,

    /// NOTE: Deprecated in favor of the format array.
    /// Maximum height in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmax: Option<i32>,

    /// NOTE: Deprecated in favor of the format array.
    /// Minimum width in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i32>,

    /// NOTE: Deprecated in favor of the format array.
    /// Minimum height in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmin: Option<i32>,

    /// Unique identifier for this banner object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Blocked banner ad types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btype: Option<Vec<BannerAdType>>,

    /// Blocked creative attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<CreativeAttribute>>,

    /// Ad position on screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<AdPosition>,

    /// Content MIME types supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<String>>,

    /// Indicates if the banner is in the top frame as opposed to an iframe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topframe: Option<TopFrameFlag>,

    /// Directions in which the banner may expand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expdir: Option<Vec<ExpandableDirection>>,

    /// List of supported API frameworks for this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<ApiFramework>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

