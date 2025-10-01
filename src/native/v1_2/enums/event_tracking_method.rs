use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Event Tracking Methods - OpenRTB Native 1.2 Section 7.7
///
/// Types of tracking methods available for events
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum EventTrackingMethod {
    /// Image-pixel tracking - URL provided will be inserted as a 1x1 pixel at the time of the event
    Img = 1,
    /// Javascript-based tracking - URL provided will be inserted as a js tag at the time of the event
    Js = 2,
    /// Exchange-specific (could include custom measurement companies such as moat, doubleverify, IAS, etc)
    ExchangeSpecific(u32),
}

impl Serialize for EventTrackingMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            EventTrackingMethod::Img => serializer.serialize_u32(1),
            EventTrackingMethod::Js => serializer.serialize_u32(2),
            EventTrackingMethod::ExchangeSpecific(val) => serializer.serialize_u32(val),
        }
    }
}

impl<'de> Deserialize<'de> for EventTrackingMethod {
    fn deserialize<D>(deserializer: D) -> Result<EventTrackingMethod, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(match value {
            1 => EventTrackingMethod::Img,
            2 => EventTrackingMethod::Js,
            v if v >= 500 => EventTrackingMethod::ExchangeSpecific(v),
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid EventTrackingMethod value: {}",
                    value
                )));
            }
        })
    }
}

