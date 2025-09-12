use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Content Taxonomy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContentTaxonomy {
    /// IAB Content Category Taxonomy 1.0
    IabContentCategory1,
    /// IAB Content Category Taxonomy 2.0
    IabContentCategory2,
}

impl Serialize for ContentTaxonomy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ContentTaxonomy::IabContentCategory1 => serializer.serialize_u32(1),
            ContentTaxonomy::IabContentCategory2 => serializer.serialize_u32(2),
        }
    }
}

impl<'de> Deserialize<'de> for ContentTaxonomy {
    fn deserialize<D>(deserializer: D) -> Result<ContentTaxonomy, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(ContentTaxonomy::IabContentCategory1),
            2 => Ok(ContentTaxonomy::IabContentCategory2),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid ContentTaxonomy value: {}",
                value
            ))),
        }
    }
}