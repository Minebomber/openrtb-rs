use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Expandable Direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExpandableDirection {
    /// Left
    Left,
    /// Right
    Right,
    /// Up
    Up,
    /// Down
    Down,
    /// Full Screen
    FullScreen,
}

impl Serialize for ExpandableDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ExpandableDirection::Left => serializer.serialize_u32(1),
            ExpandableDirection::Right => serializer.serialize_u32(2),
            ExpandableDirection::Up => serializer.serialize_u32(3),
            ExpandableDirection::Down => serializer.serialize_u32(4),
            ExpandableDirection::FullScreen => serializer.serialize_u32(5),
        }
    }
}

impl<'de> Deserialize<'de> for ExpandableDirection {
    fn deserialize<D>(deserializer: D) -> Result<ExpandableDirection, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(ExpandableDirection::Left),
            2 => Ok(ExpandableDirection::Right),
            3 => Ok(ExpandableDirection::Up),
            4 => Ok(ExpandableDirection::Down),
            5 => Ok(ExpandableDirection::FullScreen),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid ExpandableDirection value: {}",
                value
            ))),
        }
    }
}