//! Pricing information for ads

use super::enums::*;
use serde::{Deserialize, Serialize};

/// Pricing models
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PricingModel {
    /// Cost per thousand impressions
    #[serde(rename = "CPM")]
    CPM,
    /// Cost per click
    #[serde(rename = "CPC")]
    CPC,
    /// Cost per engagement
    #[serde(rename = "CPE")]
    CPE,
    /// Cost per view
    #[serde(rename = "CPV")]
    CPV,
    /// Cost per completed view
    #[serde(rename = "CPCV")]
    CPCV,
}

impl std::fmt::Display for PricingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PricingModel::CPM => write!(f, "CPM"),
            PricingModel::CPC => write!(f, "CPC"),
            PricingModel::CPE => write!(f, "CPE"),
            PricingModel::CPV => write!(f, "CPV"),
            PricingModel::CPCV => write!(f, "CPCV"),
        }
    }
}

impl std::str::FromStr for PricingModel {
    type Err = VastError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "CPM" => PricingModel::CPM,
            "CPC" => PricingModel::CPC,
            "CPE" => PricingModel::CPE,
            "CPV" => PricingModel::CPV,
            "CPCV" => PricingModel::CPCV,
            _ => return Err(VastError::InvalidPricingModel(s.to_string())),
        })
    }
}

/// Pricing information for the ad
///
/// The Pricing element provides the price that will be paid for the ad.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Pricing")]
pub struct Pricing {
    /// Pricing model (e.g., "CPM", "CPC", "CPE", "CPV")
    #[serde(rename = "@model")]
    pub model: PricingModel,

    /// ISO 4217 currency code
    #[serde(rename = "@currency")]
    pub currency: String,

    /// The price value
    #[serde(rename = "$value")]
    pub value: f64,
}
