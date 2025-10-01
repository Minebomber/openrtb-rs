//! Creative elements containing the actual ad content

// use crate::v4_3::common::ApiFramework;
// use crate::v4_3::companion::CompanionAds;
// use crate::v4_3::extensions::CreativeExtensions;
// use crate::v4_3::linear::Linear;
// use crate::v4_3::non_linear::NonLinearAds;
// use crate::v4_3::universal_ad_id::UniversalAdId;
use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Container element for one or more Creative elements
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Creatives")]
pub struct Creatives {
    /// List of creative elements
    #[xml(child = "Creative")]
    pub creative: Vec<Creative>,
}

/// Wraps each creative element within an InLine or Wrapper Ad
///
/// Each Creative contains one of Linear, NonLinearAds, or CompanionAds,
/// which defines the type of creative content.
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Creative")]
pub struct Creative {
    /// Optional identifier for the creative
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// The preferred order in which multiple Creatives should be displayed
    #[xml(attr = "sequence")]
    pub sequence: Option<u32>,

    /// Identifies the API framework used for any included API resources
    #[xml(attr = "apiFramework")]
    pub api_framework: Option<ApiFramework>,

    /// Ad-ID value of the creative
    #[xml(attr = "adId")]
    pub ad_id: Option<String>,

    /// Universal Ad ID for the creative
    #[xml(child = "UniversalAdId")]
    pub universal_ad_id: Option<UniversalAdId>,

    /// Creative extensions
    #[xml(child = "CreativeExtensions")]
    pub creative_extensions: Option<CreativeExtensions>,

    /// Linear creative content
    #[xml(child = "Linear")]
    pub linear: Option<Linear>,

    /// Non-linear creative content
    #[xml(child = "NonLinearAds")]
    pub non_linear_ads: Option<NonLinearAds>,

    /// Companion creative content
    #[xml(child = "CompanionAds")]
    pub companion_ads: Option<CompanionAds>,
}

/// The type of creative content
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
pub enum CreativeType {
    /// Linear video or audio creative
    #[xml(tag = "Linear")]
    Linear(Linear),

    /// Non-linear overlay creative
    #[xml(tag = "NonLinearAds")]
    NonLinearAds(NonLinearAds),

    /// Companion banner creative
    #[xml(tag = "CompanionAds")]
    CompanionAds(CompanionAds),
}

impl Creative {
    /// Create a new linear creative
    pub fn new_linear(linear: Linear) -> Self {
        Creative {
            id: None,
            sequence: None,
            api_framework: None,
            ad_id: None,
            universal_ad_id: None,
            creative_extensions: None,
            linear: Some(linear),
            non_linear_ads: None,
            companion_ads: None,
        }
    }

    /// Create a new non-linear creative
    pub fn new_non_linear(non_linear_ads: NonLinearAds) -> Self {
        Creative {
            id: None,
            sequence: None,
            api_framework: None,
            ad_id: None,
            universal_ad_id: None,
            creative_extensions: None,
            linear: None,
            non_linear_ads: Some(non_linear_ads),
            companion_ads: None,
        }
    }

    /// Create a new companion creative
    pub fn new_companion(companion_ads: CompanionAds) -> Self {
        Creative {
            id: None,
            sequence: None,
            api_framework: None,
            ad_id: None,
            universal_ad_id: None,
            creative_extensions: None,
            linear: None,
            non_linear_ads: None,
            companion_ads: Some(companion_ads),
        }
    }

    /// Check if this is a linear creative
    pub fn is_linear(&self) -> bool {
        self.linear.is_some()
    }

    /// Check if this is a non-linear creative
    pub fn is_non_linear(&self) -> bool {
        self.non_linear_ads.is_some()
    }

    /// Check if this is a companion creative
    pub fn is_companion(&self) -> bool {
        self.companion_ads.is_some()
    }
}
