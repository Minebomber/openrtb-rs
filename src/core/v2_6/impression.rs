use super::enums::*;
use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Impression<Ext = Value> {
    /// A unique identifier for this impression within the context of the bid request.
    pub id: String,

    /// A `Banner` object; required if this impression is offered as a banner ad opportunity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Banner>,

    /// A `Video` object; required if this impression is offered as a video ad opportunity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    /// An `Audio` object; required if this impression is offered as an audio ad opportunity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    /// A `Native` object; required if this impression is offered as a native ad opportunity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<Native>,

    /// A `Pmp` object containing any private marketplace deals in effect for this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmp: Option<Pmp>,

    /// Name of ad mediation partner, SDK technology, or player responsible for rendering ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displaymanager: Option<String>,

    /// Version of ad mediation partner, SDK technology, or player responsible for rendering ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displaymanagerver: Option<String>,

    /// 1 = the ad is interstitial or full screen, 0 = not interstitial.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instl: Option<InterstitialFlag>,

    /// Identifier for specific ad placement or ad tag that was used to initiate the auction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagid: Option<String>,

    /// Minimum bid for this impression expressed in CPM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<f64>,

    /// Currency specified using ISO-4217 alpha codes.
    #[serde(
        default = "default_bidfloorcur",
        skip_serializing_if = "Option::is_none"
    )]
    pub bidfloorcur: Option<String>,

    /// Indicates the type of browser opened upon clicking the creative in an app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clickbrowser: Option<ClickBrowser>,

    /// Flag to indicate if the impression requires secure HTTPS URL creative assets and markup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<SecureFlag>,

    /// Array of exchange-specific names of supported iframe busters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframebuster: Option<Vec<String>>,

    /// Advisory as to the number of seconds the bidder is willing to wait between the auction and
    /// the actual impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<u32>,

    /// Array of `Metric` objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<Vec<Metric>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}

fn default_bidfloorcur() -> Option<String> {
    Some("USD".to_string())
}
