use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Site<Ext = Value> {
    /// Exchange-specific site ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Site name (may be aliased at the publisher's request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Domain of the site (e.g., "mysite.foo.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Array of IAB content categories of the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Array of IAB content categories that describe the current section of the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,

    /// Array of IAB content categories that describe the current page or view of the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,

    /// URL of the page where the impression will be shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Referrer URL that caused navigation to the current page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

    /// Search string that caused navigation to the current page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Indicates if the site has been programmatically crawled or human curated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<u32>,

    /// Indicates if the site has a privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<u32>,

    /// Details about the Publisher of the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,

    /// Details about the Content within the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,

    /// Comma separated list of keywords about the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
