//! Media file definitions for video and audio content

use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Container for media files
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "MediaFiles")]
pub struct MediaFiles {
    /// List of media files
    #[xml(child = "MediaFile")]
    pub media_file: Vec<MediaFile>,

    /// Mezzanine file for server-side ad insertion
    #[xml(child = "Mezzanine")]
    pub mezzanine: Option<Mezzanine>,

    /// Interactive creative files
    #[xml(child = "InteractiveCreativeFile")]
    pub interactive_creative_files: Vec<InteractiveCreativeFile>,

    /// Closed caption files
    #[xml(child = "ClosedCaptionFiles")]
    pub closed_caption_files: Option<ClosedCaptionFiles>,
}

/// A single media file
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "MediaFile")]
pub struct MediaFile {
    /// Optional identifier
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// Method of delivery (streaming, progressive)
    #[xml(attr = "delivery")]
    pub delivery: DeliveryType,

    /// MIME type of the file
    #[xml(attr = "type")]
    pub mime_type: MimeType,

    /// Pixel width of the video
    #[xml(attr = "width")]
    pub width: Option<u32>,

    /// Pixel height of the video
    #[xml(attr = "height")]
    pub height: Option<u32>,

    /// Codec used to encode the file
    #[xml(attr = "codec")]
    pub codec: Option<String>,

    /// Average bitrate in kbps
    #[xml(attr = "bitrate")]
    pub bitrate: Option<u32>,

    /// Minimum bitrate in kbps
    #[xml(attr = "minBitrate")]
    pub min_bitrate: Option<u32>,

    /// Maximum bitrate in kbps
    #[xml(attr = "maxBitrate")]
    pub max_bitrate: Option<u32>,

    /// Whether it's scalable
    #[xml(attr = "scalable")]
    pub scalable: Option<bool>,

    /// Whether to maintain aspect ratio when scaled
    #[xml(attr = "maintainAspectRatio")]
    pub maintain_aspect_ratio: Option<bool>,

    /// API framework if applicable
    #[xml(attr = "apiFramework")]
    pub api_framework: Option<String>,

    /// File size in bytes
    #[xml(attr = "fileSize")]
    pub file_size: Option<u64>,

    /// Media rating (e.g., "PG")
    #[xml(attr = "mediaRating")]
    pub media_rating: Option<String>,

    /// URI to the media file
    #[xml(text)]
    pub uri: Uri,
}

/// Delivery method for media files
#[derive(Debug, Clone, PartialEq)]
pub enum DeliveryType {
    /// Progressive download
    Progressive,
    /// Streaming
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "progressive" => Ok(DeliveryType::Progressive),
            "streaming" => Ok(DeliveryType::Streaming),
            _ => Err(format!("Unknown delivery type: {}", s)),
        }
    }
}

/// Mezzanine file for server-side ad insertion (SSAI)
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Mezzanine")]
pub struct Mezzanine {
    /// Optional identifier
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// Codec specification
    #[xml(attr = "codec")]
    pub codec: Option<String>,

    /// Delivery protocol
    #[xml(attr = "delivery")]
    pub delivery: DeliveryType,

    /// Width in pixels
    #[xml(attr = "width")]
    pub width: u32,

    /// Height in pixels
    #[xml(attr = "height")]
    pub height: u32,

    /// MIME type
    #[xml(attr = "type")]
    pub mime_type: MimeType,

    /// Bitrate in kbps
    #[xml(attr = "bitrate")]
    pub bitrate: Option<u32>,

    /// Minimum bitrate in kbps
    #[xml(attr = "minBitrate")]
    pub min_bitrate: Option<u32>,

    /// Maximum bitrate in kbps
    #[xml(attr = "maxBitrate")]
    pub max_bitrate: Option<u32>,

    /// File size in bytes
    #[xml(attr = "fileSize")]
    pub file_size: Option<u64>,

    /// URI to the mezzanine file
    #[xml(text)]
    pub uri: Uri,
}

/// Interactive creative file
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "InteractiveCreativeFile")]
pub struct InteractiveCreativeFile {
    /// MIME type
    #[xml(attr = "type")]
    pub mime_type: Option<MimeType>,

    /// API framework
    #[xml(attr = "apiFramework")]
    pub api_framework: Option<String>,

    /// Verification parameters
    #[xml(attr = "variableDuration")]
    pub variable_duration: Option<bool>,

    /// URI to the interactive file
    #[xml(text)]
    pub uri: Uri,
}

/// Container for closed caption files
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "ClosedCaptionFiles")]
pub struct ClosedCaptionFiles {
    /// List of closed caption files
    #[xml(child = "ClosedCaptionFile")]
    pub closed_caption_file: Vec<ClosedCaptionFile>,
}

/// A closed caption file
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "ClosedCaptionFile")]
pub struct ClosedCaptionFile {
    /// MIME type
    #[xml(attr = "type")]
    pub mime_type: Option<MimeType>,

    /// Language code
    #[xml(attr = "language")]
    pub language: Option<String>,

    /// URI to the caption file
    #[xml(text)]
    pub uri: Uri,
}
