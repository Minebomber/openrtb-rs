use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This is the top-level bid response object.
///
/// The `id` attribute is a reflection of the bid request ID for logging purposes. Similarly, `bidid`
/// is an optional response tracking ID for bidders. If specified, it can be included in the subsequent
/// win notice call if the bidder wins. At least one `seatbid` object is required, which contains at
/// least one bid for an impression.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct BidResponse<Ext = Value> {
    /// ID of the bid request to which this is a response.
    pub id: String,

    /// Array of `SeatBid` objects; 1+ required if a bid is to be made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seatbid: Option<Vec<SeatBid>>,

    /// Bidder generated response ID to assist with logging/tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidid: Option<String>,

    /// Bid currency specified using ISO-4217 alpha codes.
    #[serde(default = "default_cur", skip_serializing_if = "Option::is_none")]
    pub cur: Option<String>,

    /// Optional feature to allow a bidder to set data in the exchange's cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customdata: Option<String>,

    /// Reason for not bidding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbr: Option<u32>,

    /// Placeholder for bidder-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}

fn default_cur() -> Option<String> {
    Some("USD".to_string())
}
