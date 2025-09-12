use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Location Service
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocationService {
    /// ip2location
    Ip2Location,
    /// Neustar (Quova)
    Neustar,
    /// MaxMind
    MaxMind,
    /// NetAcuity (Digital Element)
    NetAcuity,
}

impl Serialize for LocationService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            LocationService::Ip2Location => serializer.serialize_u32(1),
            LocationService::Neustar => serializer.serialize_u32(2),
            LocationService::MaxMind => serializer.serialize_u32(3),
            LocationService::NetAcuity => serializer.serialize_u32(4),
        }
    }
}

impl<'de> Deserialize<'de> for LocationService {
    fn deserialize<D>(deserializer: D) -> Result<LocationService, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(LocationService::Ip2Location),
            2 => Ok(LocationService::Neustar),
            3 => Ok(LocationService::MaxMind),
            4 => Ok(LocationService::NetAcuity),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid LocationService value: {}",
                value
            ))),
        }
    }
}