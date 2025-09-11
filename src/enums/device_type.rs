use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Device Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceType {
    /// Mobile/Tablet - General
    MobileTablet,
    /// Personal Computer
    PersonalComputer,
    /// Connected TV
    ConnectedTv,
    /// Phone
    Phone,
    /// Tablet
    Tablet,
    /// Connected Device
    ConnectedDevice,
    /// Set Top Box
    SetTopBox,
}

impl Serialize for DeviceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DeviceType::MobileTablet => serializer.serialize_u32(1),
            DeviceType::PersonalComputer => serializer.serialize_u32(2),
            DeviceType::ConnectedTv => serializer.serialize_u32(3),
            DeviceType::Phone => serializer.serialize_u32(4),
            DeviceType::Tablet => serializer.serialize_u32(5),
            DeviceType::ConnectedDevice => serializer.serialize_u32(6),
            DeviceType::SetTopBox => serializer.serialize_u32(7),
        }
    }
}

impl<'de> Deserialize<'de> for DeviceType {
    fn deserialize<D>(deserializer: D) -> Result<DeviceType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(DeviceType::MobileTablet),
            2 => Ok(DeviceType::PersonalComputer),
            3 => Ok(DeviceType::ConnectedTv),
            4 => Ok(DeviceType::Phone),
            5 => Ok(DeviceType::Tablet),
            6 => Ok(DeviceType::ConnectedDevice),
            7 => Ok(DeviceType::SetTopBox),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid DeviceType value: {}",
                value
            ))),
        }
    }
}

