//! Pricing information for ads

use hard_xml::{XmlRead, XmlWrite};

use super::enums::*;

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
