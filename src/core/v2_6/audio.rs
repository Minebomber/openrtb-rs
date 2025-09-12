use super::enums::*;
use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object represents an audio type impression.
///
/// Many of the fields are non-essential for minimally viable transactions, but are included to
/// offer fine control when needed. Audio in OpenRTB generally assumes compliance with the DAAST standard.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Audio {
    /// Content MIME types supported (e.g., "audio/mp4").
    pub mimes: Vec<String>,

    /// Minimum audio ad duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minduration: Option<u32>,

    /// Maximum audio ad duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<u32>,

    /// Array of supported audio protocols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<u32>>,

    /// Indicates the start delay in seconds for pre-roll, mid-roll, or post-roll ad placements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdelay: Option<u32>,

    /// If multiple ad impressions are offered in the same bid request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u32>,

    /// Blocked creative attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<u32>>,

    /// Maximum extended ad duration if extension is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxextended: Option<u32>,

    /// Minimum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minbitrate: Option<u32>,

    /// Maximum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxbitrate: Option<u32>,

    /// Supported delivery methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<ContentDeliveryMethod>>,

    /// Array of `Banner` objects if companion ads are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companionad: Option<Vec<Banner>>,

    /// List of supported API frameworks for this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<u32>>,

    /// Supported DAAST companion ad types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companiontype: Option<Vec<u32>>,

    /// The maximum number of ads that can be played in an ad pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxseq: Option<u32>,

    /// Type of audio feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<u32>,

    /// Indicates if the ad is stitched with audio content or delivered independently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stitched: Option<u32>,

    /// Volume normalization modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvol: Option<u32>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
