//! Media file definitions for video and audio content

use super::*;
use serde::{Deserialize, Serialize};

/// Delivery method for media files
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeliveryType {
    /// Progressive download
    #[serde(rename = "progressive")]
    Progressive,
    /// Streaming
    #[serde(rename = "streaming")]
    Streaming,
}

impl std::fmt::Display for DeliveryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeliveryType::Progressive => write!(f, "progressive"),
            DeliveryType::Streaming => write!(f, "streaming"),
        }
    }
}

impl std::str::FromStr for DeliveryType {
    type Err = VastError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "progressive" => DeliveryType::Progressive,
            "streaming" => DeliveryType::Streaming,
            _ => return Err(VastError::InvalidDeliveryType(s.to_string())),
        })
    }
}

/// Container for media files
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "MediaFiles")]
pub struct MediaFiles {
    /// List of media files
    #[serde(rename = "MediaFile", default, skip_serializing_if = "Vec::is_empty")]
    pub media_file: Vec<MediaFile>,

    /// Mezzanine file for server-side ad insertion
    #[serde(rename = "Mezzanine", skip_serializing_if = "Option::is_none")]
    pub mezzanine: Option<Mezzanine>,

    /// Interactive creative files
    #[serde(
        rename = "InteractiveCreativeFile",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub interactive_creative_files: Vec<InteractiveCreativeFile>,

    /// Closed caption files
    #[serde(rename = "ClosedCaptionFiles", skip_serializing_if = "Option::is_none")]
    pub closed_caption_files: Option<ClosedCaptionFiles>,
}

/// A single media file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "MediaFile")]
pub struct MediaFile {
    /// Optional identifier
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Method of delivery (streaming, progressive)
    #[serde(rename = "@delivery")]
    pub delivery: DeliveryType,

    /// MIME type of the file
    #[serde(rename = "@type")]
    pub mime_type: MimeType,

    /// Pixel width of the video
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// Pixel height of the video
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    /// Codec used to encode the file
    #[serde(rename = "@codec", skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,

    /// Average bitrate in kbps
    #[serde(rename = "@bitrate", skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,

    /// Minimum bitrate in kbps
    #[serde(rename = "@minBitrate", skip_serializing_if = "Option::is_none")]
    pub min_bitrate: Option<u32>,

    /// Maximum bitrate in kbps
    #[serde(rename = "@maxBitrate", skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<u32>,

    /// Whether it's scalable
    #[serde(rename = "@scalable", skip_serializing_if = "Option::is_none")]
    pub scalable: Option<bool>,

    /// Whether to maintain aspect ratio when scaled
    #[serde(
        rename = "@maintainAspectRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintain_aspect_ratio: Option<bool>,

    /// API framework if applicable
    #[serde(rename = "@apiFramework", skip_serializing_if = "Option::is_none")]
    pub api_framework: Option<String>,

    /// File size in bytes
    #[serde(rename = "@fileSize", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,

    /// Media rating (e.g., "PG")
    #[serde(rename = "@mediaRating", skip_serializing_if = "Option::is_none")]
    pub media_rating: Option<String>,

    /// URI to the media file
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Mezzanine file for server-side ad insertion (SSAI)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Mezzanine")]
pub struct Mezzanine {
    /// Optional identifier
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Codec specification
    #[serde(rename = "@codec", skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,

    /// Delivery protocol
    #[serde(rename = "@delivery")]
    pub delivery: DeliveryType,

    /// Width in pixels
    #[serde(rename = "@width")]
    pub width: u32,

    /// Height in pixels
    #[serde(rename = "@height")]
    pub height: u32,

    /// MIME type
    #[serde(rename = "@type")]
    pub mime_type: MimeType,

    /// Bitrate in kbps
    #[serde(rename = "@bitrate", skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,

    /// Minimum bitrate in kbps
    #[serde(rename = "@minBitrate", skip_serializing_if = "Option::is_none")]
    pub min_bitrate: Option<u32>,

    /// Maximum bitrate in kbps
    #[serde(rename = "@maxBitrate", skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<u32>,

    /// File size in bytes
    #[serde(rename = "@fileSize", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,

    /// URI to the mezzanine file
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Interactive creative file
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "InteractiveCreativeFile")]
pub struct InteractiveCreativeFile {
    /// MIME type
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<MimeType>,

    /// API framework
    #[serde(rename = "@apiFramework", skip_serializing_if = "Option::is_none")]
    pub api_framework: Option<String>,

    /// Verification parameters
    #[serde(rename = "@variableDuration", skip_serializing_if = "Option::is_none")]
    pub variable_duration: Option<bool>,

    /// URI to the interactive file
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Container for closed caption files
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ClosedCaptionFiles")]
pub struct ClosedCaptionFiles {
    /// List of closed caption files
    #[serde(
        rename = "ClosedCaptionFile",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub closed_caption_file: Vec<ClosedCaptionFile>,
}

/// A closed caption file
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ClosedCaptionFile")]
pub struct ClosedCaptionFile {
    /// MIME type
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<MimeType>,

    /// Language code
    #[serde(rename = "@language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// URI to the caption file
    #[serde(rename = "$value")]
    pub uri: Uri,
}
