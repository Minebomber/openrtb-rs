use super::enums::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object encapsulates various methods for specifying a geographic location.
///
/// When subordinate to a Device object, it indicates the location of the device which can also be
/// interpreted as the user's current location. When subordinate to a User object, it indicates the
/// location of the user's home base.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Geo<Ext = Value> {
    /// Latitude from -90.0 to +90.0, where negative is south.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,

    /// Longitude from -180.0 to +180.0, where negative is west.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,

    /// Source of location data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<LocationType>,

    /// Estimated location accuracy in meters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<u32>,

    /// Number of seconds since this geolocation fix was established.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastfix: Option<u32>,

    /// Service or provider used to determine geolocation from IP address if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipservice: Option<LocationService>,

    /// Country code using ISO-3166-1-alpha-3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Region code using ISO-3166-2; 2-letter state code if USA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Region of a country using FIPS 10-4 notation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regionfips104: Option<String>,

    /// Google metro code; similar to but not exactly Nielsen DMAs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metro: Option<String>,

    /// City using United Nations Code for Trade & Transport Locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Zip or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,

    /// Local time as the number +/- of minutes from UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utcoffset: Option<u32>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
