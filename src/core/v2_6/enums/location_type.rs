use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Location Type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocationType {
    /// GPS/Location Services
    GpsLocationServices,
    /// IP Address
    IpAddress,
    /// User provided (e.g. registration data)
    UserProvided,
}

impl Serialize for LocationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            LocationType::GpsLocationServices => serializer.serialize_u32(1),
            LocationType::IpAddress => serializer.serialize_u32(2),
            LocationType::UserProvided => serializer.serialize_u32(3),
        }
    }
}

impl<'de> Deserialize<'de> for LocationType {
    fn deserialize<D>(deserializer: D) -> Result<LocationType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(LocationType::GpsLocationServices),
            2 => Ok(LocationType::IpAddress),
            3 => Ok(LocationType::UserProvided),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid LocationType value: {}",
                value
            ))),
        }
    }
}