use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object constitutes a specific deal that was struck *a priori* between a buyer and a seller.
///
/// Its presence with the Pmp collection indicates that this impression is available under the terms
/// of that deal.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Deal<Ext = Value> {
    /// A unique identifier for the direct deal.
    pub id: String,

    /// Minimum bid for this impression expressed in CPM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<f64>,

    /// Currency specified using ISO-4217 alpha codes.
    #[serde(
        default = "default_bidfloorcur",
        skip_serializing_if = "Option::is_none"
    )]
    pub bidfloorcur: Option<String>,

    /// Auction type. If "1", then first price auction. If "2", then second price plus auction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<u32>,

    /// Allowed list of buyer seats allowed to bid on this deal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wseat: Option<Vec<String>>,

    /// Array of advertiser domains (e.g., advertiser.com) allowed to bid on this deal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wadv: Option<Vec<String>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}

fn default_bidfloorcur() -> Option<String> {
    Some("USD".to_string())
}
