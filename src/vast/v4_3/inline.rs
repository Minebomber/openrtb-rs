//! InLine ad element containing all creative files and tracking

use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// InLine ad containing all necessary files, URIs, and tracking resources.
///
/// The InLine element contains all the information necessary to display the ad,
/// including creative files, tracking URIs, and companion ads.
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "InLine")]
pub struct InLine {
    /// Name of the ad server that returned the ad
    #[xml(child = "AdSystem")]
    pub ad_system: AdSystem,

    /// Common name of the ad
    #[xml(child = "AdTitle")]
    pub ad_title: Option<AdTitle>,

    /// Name of the advertiser as defined by the ad serving party
    #[xml(child = "Advertiser")]
    pub advertiser: Option<Advertiser>,

    /// One or more URIs that directs the browser to a tracking resource file that the ad server uses to track impressions
    #[xml(child = "Impression")]
    pub impressions: Vec<Impression>,

    /// Descriptive name of the ad categories
    #[xml(child = "Category")]
    pub categories: Vec<Category>,

    /// A longer description of the ad
    #[xml(child = "Description")]
    pub description: Option<Description>,

    /// Survey URI
    #[xml(child = "Survey")]
    pub survey: Option<Survey>,

    /// URI to any resource file having to do with collecting survey data
    #[xml(child = "Error")]
    pub errors: Vec<ErrorElement>,

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

    /// Creative elements for the ad
    #[xml(child = "Creatives")]
    pub creatives: Creatives,
}

/// Identifies the ad server that returned the ad
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "AdSystem")]
pub struct AdSystem {
    /// The version of the ad system
    #[xml(attr = "version")]
    pub version: Option<String>,

    /// Name of the ad system
    #[xml(text)]
    pub name: String,
}

/// Advertiser information
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Advertiser")]
pub struct Advertiser {
    /// Optional identifier for the advertiser
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// Name of the advertiser
    #[xml(text)]
    pub name: String,
}

/// URI to track an impression
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Impression")]
pub struct Impression {
    /// Optional identifier for the impression
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// The impression tracking URI
    #[xml(text)]
    pub uri: Uri,
}

/// Ad title wrapper
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "AdTitle")]
pub struct AdTitle {
    #[xml(text)]
    pub value: String,
}

/// Description wrapper
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Description")]
pub struct Description {
    #[xml(text)]
    pub value: String,
}

/// Survey wrapper
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Survey")]
pub struct Survey {
    #[xml(text)]
    pub value: String,
}

/// Error element wrapper
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Error")]
pub struct ErrorElement {
    #[xml(text)]
    pub uri: Uri,
}
