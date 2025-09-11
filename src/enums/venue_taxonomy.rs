use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Venue Taxonomy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VenueTaxonomy {
    /// AdCom 1.0
    AdCom1,
}

impl Serialize for VenueTaxonomy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            VenueTaxonomy::AdCom1 => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for VenueTaxonomy {
    fn deserialize<D>(deserializer: D) -> Result<VenueTaxonomy, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(VenueTaxonomy::AdCom1),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid VenueTaxonomy value: {}",
                value
            ))),
        }
    }
}