use super::*;
use crate::enums::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object provides information pertaining to the device through which the user is interacting.
///
/// Device information includes its hardware, platform, location, and carrier data. The device can
/// refer to a mobile handset, a desktop computer, set top box, or other digital device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Device {
    /// Browser user agent string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ua: Option<String>,

    /// Location of the device assumed to be the user's current location defined by a `Geo` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,

    /// Standard "Do Not Track" flag as set in the header by the browser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnt: Option<DoNotTrackFlag>,

    /// "Limit Ad Tracking" signal commercially endorsed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmt: Option<LimitAdTrackingFlag>,

    /// IPv4 address closest to device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// IP address closest to device as IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,

    /// The general type of device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devicetype: Option<DeviceType>,

    /// Device make (e.g., "Apple").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,

    /// Device model (e.g., "iPhone").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Device operating system (e.g., "iOS").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    /// Device operating system version (e.g., "3.1.2").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osv: Option<String>,

    /// Hardware version of the device (e.g., "5S" for iPhone 5S).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwv: Option<String>,

    /// Physical height of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u32>,

    /// Physical width of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u32>,

    /// Screen size as pixels per linear inch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<u32>,

    /// The ratio of physical pixels to device independent pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pxratio: Option<f64>,

    /// Support for JavaScript.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<JavaScriptFlag>,

    /// Indicates if the geolocation API will be available to JavaScript code running in the banner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geofetch: Option<GeoFetchFlag>,

    /// Version of Flash supported by the browser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flashver: Option<String>,

    /// Browser language using ISO-639-1-alpha-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Carrier or ISP (e.g., "VERIZON") using exchange curated string names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// Mobile carrier as the concatenated MCC-MNC code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mccmnc: Option<String>,

    /// Network connection type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectiontype: Option<ConnectionType>,

    /// ID sanctioned for advertiser use in the clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifa: Option<String>,

    /// Hardware device ID (e.g., IMEI); hashed via SHA1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didsha1: Option<String>,

    /// Hardware device ID (e.g., IMEI); hashed via MD5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didmd5: Option<String>,

    /// Platform device ID (e.g., Android ID); hashed via SHA1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpidsha1: Option<String>,

    /// Platform device ID (e.g., Android ID); hashed via MD5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpidmd5: Option<String>,

    /// MAC address of the device; hashed via SHA1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macsha1: Option<String>,

    /// MAC address of the device; hashed via MD5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macmd5: Option<String>,

    /// Structured user agent information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sua: Option<UserAgent>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
