//! InLine ad element containing all creative files and tracking

use super::*;
use serde::{Deserialize, Serialize};

/// InLine ad containing all necessary files, URIs, and tracking resources.
///
/// The InLine element contains all the information necessary to display the ad,
/// including creative files, tracking URIs, and companion ads.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "InLine")]
pub struct InLine {
    /// Name of the ad server that returned the ad
    #[serde(rename = "AdSystem")]
    pub ad_system: AdSystem,

    /// Common name of the ad
    #[serde(rename = "AdTitle", skip_serializing_if = "Option::is_none")]
    pub ad_title: Option<AdTitle>,

    /// Name of the advertiser as defined by the ad serving party
    #[serde(rename = "Advertiser", skip_serializing_if = "Option::is_none")]
    pub advertiser: Option<Advertiser>,

    /// One or more URIs that directs the browser to a tracking resource file that the ad server uses to track impressions
    #[serde(rename = "Impression", default, skip_serializing_if = "Vec::is_empty")]
    pub impressions: Vec<Impression>,

    /// Descriptive name of the ad categories
    #[serde(rename = "Category", default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<Category>,

    /// A longer description of the ad
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,

    /// Survey URI
    #[serde(rename = "Survey", skip_serializing_if = "Option::is_none")]
    pub survey: Option<Survey>,

    /// URI to any resource file having to do with collecting survey data
    #[serde(rename = "Error", default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorElement>,

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

    /// Creative elements for the ad
    #[serde(rename = "Creatives")]
    pub creatives: Creatives,
}

/// Identifies the ad server that returned the ad
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "AdSystem")]
pub struct AdSystem {
    /// The version of the ad system
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Name of the ad system
    #[serde(rename = "$value")]
    pub name: String,
}

/// Advertiser information
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Advertiser")]
pub struct Advertiser {
    /// Optional identifier for the advertiser
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of the advertiser
    #[serde(rename = "$value")]
    pub name: String,
}

/// URI to track an impression
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Impression")]
pub struct Impression {
    /// Optional identifier for the impression
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The impression tracking URI
    #[serde(rename = "$value")]
    pub uri: Uri,
}

/// Ad title wrapper
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "AdTitle")]
pub struct AdTitle {
    #[serde(rename = "$value")]
    pub value: String,
}

/// Description wrapper
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Description")]
pub struct Description {
    #[serde(rename = "$value")]
    pub value: String,
}

/// Survey wrapper
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Survey")]
pub struct Survey {
    #[serde(rename = "$value")]
    pub value: String,
}

/// Error element wrapper
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Error")]
pub struct ErrorElement {
    #[serde(rename = "$value")]
    pub uri: Uri,
}
