use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A bid response can contain multiple `SeatBid` objects, each on behalf of a different bidder seat.
///
/// Since a bid request can include multiple impressions, each `SeatBid` can contain multiple bids each
/// pertaining to a different impression.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeatBid {
    /// Array of 1+ `Bid` objects each related to an impression.
    pub bid: Vec<Bid>,

    /// ID of the buyer seat (e.g., advertiser, agency) on whose behalf this bid is made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat: Option<String>,

    /// 0 = impressions can be won individually; 1 = impressions must be won or lost as a group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<u32>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
