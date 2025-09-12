use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object should be included if the ad supported content is a non-browser application
/// as opposed to a website.
///
/// A bid request must not contain both an App and a Site object. At a minimum, it is useful
/// to provide an App ID or bundle, but this is not strictly required.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct App {
    /// Exchange-specific app ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// App name (may be aliased at the publisher's request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A platform-specific application identifier intended to be unique to the app and independent of the exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,

    /// Domain of the app (e.g., "mygame.foo.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// App store URL for an installed app; for QAG 1.5 compliance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storeurl: Option<String>,

    /// Array of IAB content categories of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Array of IAB content categories that describe the current section of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,

    /// Array of IAB content categories that describe the current page or view of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,

    /// Application version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// Indicates if the app has a privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<u32>,

    /// 0 = app is free, 1 = the app is a paid version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<u32>,

    /// Details about the Publisher of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,

    /// Details about the Content within the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,

    /// Comma separated list of keywords about the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
