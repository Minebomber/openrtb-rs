//! Wrapper ad element for VAST redirects

use super::*;
use serde::{Deserialize, Serialize};

/// Wrapper element that redirects the video player to another VAST response.
///
/// A Wrapper element is used to redirect the video player to another server for the actual ad,
/// while still allowing the current server to track impressions and other events.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Wrapper")]
pub struct Wrapper {
    /// Indicates whether subsequent wrappers after a requested VAST response is allowed
    #[serde(
        rename = "@followAdditionalWrappers",
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_additional_wrappers: Option<bool>,

    /// Indicates whether multiple ads are allowed in the requested VAST response
    #[serde(rename = "@allowMultipleAds", skip_serializing_if = "Option::is_none")]
    pub allow_multiple_ads: Option<bool>,

    /// Indicates whether the fallback ads are to be used
    #[serde(rename = "@fallbackOnNoAd", skip_serializing_if = "Option::is_none")]
    pub fallback_on_no_ad: Option<bool>,

    /// Name of the ad server that returned the ad
    #[serde(rename = "AdSystem")]
    pub ad_system: WrapperAdSystem,

    /// URI to the VAST response that contains the actual ad
    #[serde(rename = "VASTAdTagURI")]
    pub vast_ad_tag_uri: VASTAdTagURI,

    /// One or more URIs that directs the browser to a tracking resource
    #[serde(rename = "Impression", default, skip_serializing_if = "Vec::is_empty")]
    pub impressions: Vec<WrapperImpression>,

    /// URI to any resource file having to do with collecting survey data
    #[serde(rename = "Error", default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<WrapperError>,

    /// Viewable impression tracking
    #[serde(rename = "ViewableImpression", skip_serializing_if = "Option::is_none")]
    pub viewable_impression: Option<ViewableImpression>,

    /// Ad verifications for OMID and other verification vendors
    #[serde(rename = "AdVerifications", skip_serializing_if = "Option::is_none")]
    pub ad_verifications: Option<AdVerifications>,

    /// Container for Extensions
    #[serde(rename = "Extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    /// Pricing information
    #[serde(rename = "Pricing", skip_serializing_if = "Option::is_none")]
    pub pricing: Option<Pricing>,

    /// Creative elements that can override or supplement wrapped ad creative
    #[serde(rename = "Creatives", skip_serializing_if = "Option::is_none")]
    pub creatives: Option<Creatives>,

    /// Blocked advertiser domains
    #[serde(
        rename = "BlockedAdCategories",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_ad_categories: Vec<BlockedAdCategories>,
}

/// Ad system for wrapper ads
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "AdSystem")]
pub struct WrapperAdSystem {
    /// The version of the ad system
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Name of the ad system
    #[serde(rename = "$value")]
    pub name: String,
}

/// URI to a VAST response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "VASTAdTagURI")]
pub struct VASTAdTagURI {
    /// URI to the VAST response
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Impression tracking for wrapper ads
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Impression")]
pub struct WrapperImpression {
    /// Optional identifier for the impression
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The impression tracking URI
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Blocked ad categories
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "BlockedAdCategories")]
pub struct BlockedAdCategories {
    /// Category authority
    #[serde(rename = "@authority", skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,

    /// Category codes
    #[serde(rename = "$value")]
    pub categories: String,
}

/// Error element wrapper for Wrapper ads
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Error")]
pub struct WrapperError {
    #[serde(rename = "$value")]
    pub uri: Uri,
}
