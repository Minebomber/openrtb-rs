use super::enums::*;
use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Structured User-Agent information defined by the IAB Tech Lab for use in OpenRTB
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAgent<Ext = Value> {
    /// Each `Brand` object identifies a browser or similar software component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browsers: Option<Vec<Brand>>,

    /// Device model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Brand>,

    /// If the browser is running on a mobile device, this object represents the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<u32>,

    /// 1 if the agent prefers a mobile version of the content, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    /// Device's major binary architecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitness: Option<String>,

    /// Device's bitness (e.g., "64" for a 64-bit CPU architecture).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Source of the User-Agent data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<UserAgentSource>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
