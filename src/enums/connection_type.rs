use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Connection Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionType {
    /// Unknown
    Unknown,
    /// Ethernet
    Ethernet,
    /// WIFI
    Wifi,
    /// Cellular Network  Unknown Generation
    CellularUnknown,
    /// Cellular Network  2G
    Cellular2G,
    /// Cellular Network  3G
    Cellular3G,
    /// Cellular Network  4G
    Cellular4G,
}

impl Serialize for ConnectionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ConnectionType::Unknown => serializer.serialize_u32(0),
            ConnectionType::Ethernet => serializer.serialize_u32(1),
            ConnectionType::Wifi => serializer.serialize_u32(2),
            ConnectionType::CellularUnknown => serializer.serialize_u32(3),
            ConnectionType::Cellular2G => serializer.serialize_u32(4),
            ConnectionType::Cellular3G => serializer.serialize_u32(5),
            ConnectionType::Cellular4G => serializer.serialize_u32(6),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectionType {
    fn deserialize<D>(deserializer: D) -> Result<ConnectionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(ConnectionType::Unknown),
            1 => Ok(ConnectionType::Ethernet),
            2 => Ok(ConnectionType::Wifi),
            3 => Ok(ConnectionType::CellularUnknown),
            4 => Ok(ConnectionType::Cellular2G),
            5 => Ok(ConnectionType::Cellular3G),
            6 => Ok(ConnectionType::Cellular4G),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid ConnectionType value: {}",
                value
            ))),
        }
    }
}

