use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Event Types - OpenRTB Native 1.2 Section 7.6
///
/// Types of events available for tracking
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum EventType {
    /// Impression
    Impression = 1,
    /// Visible impression using MRC definition at 50% in view for 1 second
    ViewableMrc50 = 2,
    /// 100% in view for 1 second (ie GroupM standard)
    ViewableMrc100 = 3,
    /// Visible impression for video using MRC definition at 50% in view for 2 seconds
    ViewableVideo50 = 4,
    /// Exchange-specific value (500+)
    ExchangeSpecific(u32),
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            EventType::Impression => serializer.serialize_u32(1),
            EventType::ViewableMrc50 => serializer.serialize_u32(2),
            EventType::ViewableMrc100 => serializer.serialize_u32(3),
            EventType::ViewableVideo50 => serializer.serialize_u32(4),
            EventType::ExchangeSpecific(val) => serializer.serialize_u32(val),
        }
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<EventType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(match value {
            1 => EventType::Impression,
            2 => EventType::ViewableMrc50,
            3 => EventType::ViewableMrc100,
            4 => EventType::ViewableVideo50,
            v if v >= 500 => EventType::ExchangeSpecific(v),
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid EventType value: {}",
                    value
                )))
            }
        })
    }
}