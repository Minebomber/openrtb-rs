//! Ad element and related types

use super::*;
use serde::{Deserialize, Serialize};

/// Container for a single ad within a VAST response.
///
/// An Ad element contains all data necessary to display an ad. Each Ad represents either
/// an InLine ad (containing all creative files and tracking URIs) or a Wrapper ad
/// (redirecting to another VAST response).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Ad")]
pub struct Ad {
    /// Unique identifier for the ad
    #[serde(rename = "@id")]
    pub id: String,

    /// Indicates the sequence number in which the ad should be displayed within an ad pod
    #[serde(rename = "@sequence", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u32>,

    /// Indicates whether the ad is conditional (e.g., based on user opt-in)
    #[serde(rename = "@conditionalAd", skip_serializing_if = "Option::is_none")]
    pub conditional_ad: Option<bool>,

    /// InLine ad content
    #[serde(rename = "InLine", skip_serializing_if = "Option::is_none")]
    pub inline: Option<InLine>,

    /// Wrapper ad content
    #[serde(rename = "Wrapper", skip_serializing_if = "Option::is_none")]
    pub wrapper: Option<Wrapper>,
}

impl Ad {
    /// Create a new inline ad
    pub fn new_inline(id: String, inline: InLine) -> Self {
        Ad {
            id,
            sequence: None,
            conditional_ad: None,
            inline: Some(inline),
            wrapper: None,
        }
    }

    /// Create a new wrapper ad
    pub fn new_wrapper(id: String, wrapper: Wrapper) -> Self {
        Ad {
            id,
            sequence: None,
            conditional_ad: None,
            inline: None,
            wrapper: Some(wrapper),
        }
    }

    /// Check if this is an inline ad
    pub fn is_inline(&self) -> bool {
        self.inline.is_some()
    }

    /// Check if this is a wrapper ad
    pub fn is_wrapper(&self) -> bool {
        self.wrapper.is_some()
    }
}

