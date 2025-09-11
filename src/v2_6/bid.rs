use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A `SeatBid` object contains one or more `Bid` objects, each of which relates to a specific impression
/// in the bid request via the `impid` attribute and constitutes an offer to buy that impression for a
/// given price.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bid {
    /// Bidder generated bid ID to assist with logging/tracking.
    pub id: String,

    /// ID of the `Imp` object in the related bid request.
    pub impid: String,

    /// Bid price expressed as CPM although the actual transaction is for a unit impression.
    pub price: f64,

    /// Win notice URL called by the exchange if the bid wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nurl: Option<String>,

    /// Billing notice URL called by the exchange when a winning bid becomes billable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burl: Option<String>,

    /// Loss notice URL called by the exchange when a bid is known to have been lost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lurl: Option<String>,

    /// Ad markup. XHTML if a response to a Banner object, or VAST XML if a response to a Video object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<String>,

    /// ID of a preloaded ad to be served if the bid wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adid: Option<String>,

    /// Advertiser domain for block list checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adomain: Option<Vec<String>>,

    /// A platform-specific application identifier intended to be unique to the app and independent of the exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,

    /// URL without cache-busting to an image that is representative of the content of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iurl: Option<String>,

    /// Campaign ID to assist with ad quality checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,

    /// Creative ID to assist with ad quality checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crid: Option<String>,

    /// Tactic ID to enable buyers to label bids for reporting to the exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tactic: Option<String>,

    /// IAB content categories of the creative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Set of attributes describing the creative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr: Option<Vec<u32>>,

    /// List of supported APIs for the markup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apis: Option<Vec<u32>>,

    /// API required by the markup if applicable.
    #[deprecated(note = "Deprecated in favor of `apis`.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<u32>,

    /// Video response protocol of the markup if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<u32>,

    /// Creative media rating per IQG guidelines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<u32>,

    /// Language of the creative using ISO-639-1-alpha-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Reference to the `Deal.id` from the bid request if this bid pertains to a private marketplace direct deal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dealid: Option<String>,

    /// Width of the creative in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u32>,

    /// Height of the creative in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u32>,

    /// Relative width of the creative when expressing size as a ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<u32>,

    /// Relative height of the creative when expressing size as a ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<u32>,

    /// Advisory as to the number of seconds the bidder is willing to wait between the auction and the actual impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<u32>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
