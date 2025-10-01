//! Pricing information for ads

use hard_xml::{XmlRead, XmlWrite};

/// Pricing information for the ad
///
/// The Pricing element provides the price that will be paid for the ad.
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Pricing")]
pub struct Pricing {
    /// Pricing model (e.g., "CPM", "CPC", "CPE", "CPV")
    #[xml(attr = "model")]
    pub model: PricingModel,

    /// ISO 4217 currency code
    #[xml(attr = "currency")]
    pub currency: String,

    /// The price value
    #[xml(text)]
    pub value: f64,
}

/// Pricing models
#[derive(Debug, Clone, PartialEq)]
pub enum PricingModel {
    /// Cost per thousand impressions
    CPM,
    /// Cost per click
    CPC,
    /// Cost per engagement
    CPE,
    /// Cost per view
    CPV,
    /// Cost per completed view
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CPM" => Ok(PricingModel::CPM),
            "CPC" => Ok(PricingModel::CPC),
            "CPE" => Ok(PricingModel::CPE),
            "CPV" => Ok(PricingModel::CPV),
            "CPCV" => Ok(PricingModel::CPCV),
            _ => Err(format!("Unknown pricing model: {}", s)),
        }
    }
}
