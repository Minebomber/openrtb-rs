use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// API Frameworks
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ApiFramework {
    /// VPAID 1.0
    Vpaid1,
    /// VPAID 2.0
    Vpaid2,
    /// MRAID-1
    Mraid1,
    /// ORMMA
    Ormma,
    /// MRAID-2
    Mraid2,
    /// MRAID-3
    Mraid3,
    /// OMID-1
    Omid1,
}

impl Serialize for ApiFramework {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ApiFramework::Vpaid1 => serializer.serialize_u32(1),
            ApiFramework::Vpaid2 => serializer.serialize_u32(2),
            ApiFramework::Mraid1 => serializer.serialize_u32(3),
            ApiFramework::Ormma => serializer.serialize_u32(4),
            ApiFramework::Mraid2 => serializer.serialize_u32(5),
            ApiFramework::Mraid3 => serializer.serialize_u32(6),
            ApiFramework::Omid1 => serializer.serialize_u32(7),
        }
    }
}

impl<'de> Deserialize<'de> for ApiFramework {
    fn deserialize<D>(deserializer: D) -> Result<ApiFramework, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(ApiFramework::Vpaid1),
            2 => Ok(ApiFramework::Vpaid2),
            3 => Ok(ApiFramework::Mraid1),
            4 => Ok(ApiFramework::Ormma),
            5 => Ok(ApiFramework::Mraid2),
            6 => Ok(ApiFramework::Mraid3),
            7 => Ok(ApiFramework::Omid1),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid ApiFramework value: {}",
                value
            ))),
        }
    }
}

