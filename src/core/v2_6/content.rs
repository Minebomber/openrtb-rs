use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object describes the content in which the impression will appear.
///
/// This may be syndicated or non-syndicated content. This object may be useful when syndicated
/// content contains impressions and does not necessarily match the publisher's general content.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Content<Ext = Value> {
    /// ID uniquely identifying the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Episode number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<u32>,

    /// Content title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Content series.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,

    /// Content season (e.g., "Season 3").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,

    /// Artist credited with the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,

    /// Genre that best describes the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,

    /// Album to which the content belongs; typically for audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,

    /// International Standard Recording Code conforming to ISO-3901.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,

    /// Details about the content `Producer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer: Option<Producer>,

    /// URL of the content, for buy-side contextualization or review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Array of IAB content categories that describe the content producer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Production quality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prodq: Option<u32>,

    /// Note: Deprecated in favor of `prodq`.
    /// Video quality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videoquality: Option<u32>,

    /// Type of content (game, video, text, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<u32>,

    /// Content rating (e.g., MPAA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentrating: Option<String>,

    /// User rating of the content (e.g., number of stars, likes, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userrating: Option<String>,

    /// Media rating per IQG guidelines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<u32>,

    /// Comma separated list of keywords describing the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// 0 = not live, 1 = content is being streamed live.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livestream: Option<u32>,

    /// 0 = indirect, 1 = direct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourcerelationship: Option<u32>,

    /// Length of content in seconds; appropriate for video or audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<u32>,

    /// Content language using ISO-639-1-alpha-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Indicator of whether or not the content is embeddable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddable: Option<u32>,

    /// Additional content data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Data>>,

    /// Content Network the content belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,

    /// Channel the content belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
