use super::*;
use crate::enums::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object represents an in-stream video impression.
///
/// Many of the fields are non-essential for minimally viable transactions, but are included to
/// offer fine control when needed. Video in OpenRTB generally assumes compliance with the VAST standard.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Video {
    /// Content MIME types supported (e.g., "video/x-ms-wmv", "video/mp4").
    pub mimes: Vec<String>,

    /// Minimum video ad duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minduration: Option<i32>,

    /// Maximum video ad duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<i32>,

    /// Array of supported video protocols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<VideoBidResponseProtocol>>,

    /// NOTE: Deprecated in favor of protocols.
    /// Supported video protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<VideoBidResponseProtocol>,

    /// Width of the video player in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height of the video player in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Indicates the start delay in seconds for pre-roll, mid-roll, or post-roll ad placements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdelay: Option<i32>,

    /// Indicates if the impression must be linear, nonlinear, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linearity: Option<VideoLinearity>,

    /// Indicates if the player will allow the video to be skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<SkipFlag>,

    /// Videos of total duration greater than this number of seconds can be skippable.
    #[serde(default = "default_skip_min", skip_serializing_if = "Option::is_none")]
    pub skipmin: Option<i32>,

    /// Number of seconds a video must play before skipping is enabled.
    #[serde(
        default = "default_skip_after",
        skip_serializing_if = "Option::is_none"
    )]
    pub skipafter: Option<i32>,

    /// If multiple ad impressions are offered in the same bid request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,

    /// Blocked creative attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<CreativeAttribute>>,

    /// Maximum extended ad duration if extension is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxextended: Option<i32>,

    /// Minimum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minbitrate: Option<i32>,

    /// Maximum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxbitrate: Option<i32>,

    /// Indicates if letter-boxing of 4:3 content into a 16:9 window is allowed.
    #[serde(
        default = "default_boxingallowed",
        skip_serializing_if = "Option::is_none"
    )]
    pub boxingallowed: Option<BoxingAllowedFlag>,

    /// Playback methods that may be in use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playbackmethod: Option<Vec<PlaybackMethod>>,

    /// The event that causes playback to end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playbackend: Option<PlaybackCessationMode>,

    /// Supported delivery methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<ContentDeliveryMethod>>,

    /// Ad position on screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<AdPosition>,

    /// Array of `Banner` objects if companion ads are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companionad: Option<Vec<Banner>>,

    /// List of supported API frameworks for this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<ApiFramework>>,

    /// Supported VAST companion ad types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companiontype: Option<Vec<CompanionType>>,

    /// The maximum number of ads that can be played in an ad pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxseq: Option<i32>,

    /// Type of video feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<FeedType>,

    /// Indicates if the ad is stitched with video content or delivered independently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stitched: Option<StitchedFlag>,

    /// Volume normalization modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvol: Option<VolumeNormalizationMode>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

