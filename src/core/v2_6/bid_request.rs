use super::enums::*;
use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Top-level bid request object
///
/// This object contains a globally unique bid request or auction ID. This `id` attribute is
/// required as is at least one `Imp` object. Other attributes in this top-level object establish
/// rules and restrictions that apply to all impressions being offered.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BidRequest<Ext = Value> {
    /// ID of the bid request, assigned by the exchange, and unique for the exchange's subsequent
    /// tracking of the responses. The exchange may use different values for different recipients.
    pub id: String,

    /// Array of `Imp` objects representing the impressions offered. At least 1 `Imp` object is required.
    pub imp: Vec<Impression>,

    /// Details via a `Site` object about the publisher's website. Only applicable and recommended for websites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,

    /// Details via an `App` object about the publisher's app (i.e., non-browser applications).
    /// Only applicable and recommended for apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,

    /// This object should be included if the ad supported content is a Digital Out-Of-Home screen.
    /// A bid request with a DOOH object must not contain a site or app object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dooh: Option<Dooh>,

    /// Details via a `Device` object about the user's device to which the impression will be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,

    /// Details via a `User` object about the human user of the device; the advertising audience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// Indicator of test mode in which auctions are not billable, where 0 = live mode, 1 = test mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test: Option<TestMode>,

    /// Auction type, where 1 = First Price, 2 = Second Price Plus.
    /// Exchange-specific auction types can be defined using values 500 and greater.
    #[serde(default = "default_at", skip_serializing_if = "Option::is_none")]
    pub at: Option<AuctionType>,

    /// Maximum time in milliseconds the exchange allows for bids to be received including
    /// Internet latency to avoid timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmax: Option<u32>,

    /// Allowed list of buyer seats (e.g., advertisers, agencies) allowed to bid on this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wseat: Option<Vec<String>>,

    /// Block list of buyer seats (e.g., advertisers, agencies) restricted from bidding on this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bseat: Option<Vec<String>>,

    /// Flag to indicate if Exchange can verify that the impressions offered represent all or a
    /// group of the impressions available in context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allimps: Option<AllImpsFlag>,

    /// Array of allowed currencies for bids on this bid request using ISO-4217 alpha codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<Vec<String>>,

    /// Allowed list of languages for creatives using ISO-639-1-alpha-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wlang: Option<Vec<String>>,

    /// Indicates that the bidder is allowed to provide markup that will be cached by the exchange.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cacheid: Option<String>,

    /// Block list of content categories using the IAB content category taxonomy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcat: Option<Vec<String>>,

    /// Block list of advertisers by their domains (e.g., \"ford.com\").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badv: Option<Vec<String>>,

    /// Block list of apps by their platform-specific exchange-independent application identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bapp: Option<Vec<String>>,

    /// Details via a `Source` object about the inventory source and which entity makes the final decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /// Details via `Regs` object about any industry, legal, or governmental regulations in force for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regs: Option<Regs>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}

fn default_at() -> Option<AuctionType> {
    Some(AuctionType::SecondPricePlus)
}
