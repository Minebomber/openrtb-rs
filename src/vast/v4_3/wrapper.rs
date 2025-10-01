//! Wrapper ad element for VAST redirects

use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Wrapper element that redirects the video player to another VAST response.
///
/// A Wrapper element is used to redirect the video player to another server for the actual ad,
/// while still allowing the current server to track impressions and other events.
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Wrapper")]
pub struct Wrapper {
    /// Indicates whether subsequent wrappers after a requested VAST response is allowed
    #[xml(attr = "followAdditionalWrappers")]
    pub follow_additional_wrappers: Option<bool>,

    /// Indicates whether multiple ads are allowed in the requested VAST response
    #[xml(attr = "allowMultipleAds")]
    pub allow_multiple_ads: Option<bool>,

    /// Indicates whether the fallback ads are to be used
    #[xml(attr = "fallbackOnNoAd")]
    pub fallback_on_no_ad: Option<bool>,

    /// Name of the ad server that returned the ad
    #[xml(child = "AdSystem")]
    pub ad_system: WrapperAdSystem,

    /// URI to the VAST response that contains the actual ad
    #[xml(child = "VASTAdTagURI")]
    pub vast_ad_tag_uri: VASTAdTagURI,

    /// One or more URIs that directs the browser to a tracking resource
    #[xml(child = "Impression")]
    pub impressions: Vec<WrapperImpression>,

    /// URI to any resource file having to do with collecting survey data
    #[xml(child = "Error")]
    pub errors: Vec<WrapperError>,

    /// Viewable impression tracking
    #[xml(child = "ViewableImpression")]
    pub viewable_impression: Option<ViewableImpression>,

    /// Ad verifications for OMID and other verification vendors
    #[xml(child = "AdVerifications")]
    pub ad_verifications: Option<AdVerifications>,

    /// Container for Extensions
    #[xml(child = "Extensions")]
    pub extensions: Option<Extensions>,

    /// Pricing information
    #[xml(child = "Pricing")]
    pub pricing: Option<Pricing>,

    /// Creative elements that can override or supplement wrapped ad creative
    #[xml(child = "Creatives")]
    pub creatives: Option<Creatives>,

    /// Blocked advertiser domains
    #[xml(child = "BlockedAdCategories")]
    pub blocked_ad_categories: Vec<BlockedAdCategories>,
}

/// Ad system for wrapper ads
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "AdSystem")]
pub struct WrapperAdSystem {
    /// The version of the ad system
    #[xml(attr = "version")]
    pub version: Option<String>,

    /// Name of the ad system
    #[xml(text)]
    pub name: String,
}

/// URI to a VAST response
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "VASTAdTagURI")]
pub struct VASTAdTagURI {
    /// URI to the VAST response
    #[xml(text)]
    pub uri: Uri,
}

/// Impression tracking for wrapper ads
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Impression")]
pub struct WrapperImpression {
    /// Optional identifier for the impression
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// The impression tracking URI
    #[xml(text)]
    pub uri: Uri,
}

/// Blocked ad categories
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "BlockedAdCategories")]
pub struct BlockedAdCategories {
    /// Category authority
    #[xml(attr = "authority")]
    pub authority: Option<String>,

    /// Category codes
    #[xml(text)]
    pub categories: String,
}

/// Error element wrapper for Wrapper ads
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Error")]
pub struct WrapperError {
    #[xml(text)]
    pub uri: Uri,
}
