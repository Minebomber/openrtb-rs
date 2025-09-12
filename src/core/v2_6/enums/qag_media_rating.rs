use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// QAG Media Ratings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QagMediaRating {
    /// All Audiences
    AllAudiences,
    /// Everyone Over 12
    EveryoneOver12,
    /// Mature Audiences
    MatureAudiences,
}

impl Serialize for QagMediaRating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            QagMediaRating::AllAudiences => serializer.serialize_u32(1),
            QagMediaRating::EveryoneOver12 => serializer.serialize_u32(2),
            QagMediaRating::MatureAudiences => serializer.serialize_u32(3),
        }
    }
}

impl<'de> Deserialize<'de> for QagMediaRating {
    fn deserialize<D>(deserializer: D) -> Result<QagMediaRating, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(QagMediaRating::AllAudiences),
            2 => Ok(QagMediaRating::EveryoneOver12),
            3 => Ok(QagMediaRating::MatureAudiences),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid QagMediaRating value: {}",
                value
            ))),
        }
    }
}

/// IQG Media Ratings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IqgMediaRating {
    /// All Audiences
    AllAudiences,
    /// Everyone Over 12
    EveryoneOver12,
    /// Mature Audiences
    MatureAudiences,
}

impl Serialize for IqgMediaRating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            IqgMediaRating::AllAudiences => serializer.serialize_u32(1),
            IqgMediaRating::EveryoneOver12 => serializer.serialize_u32(2),
            IqgMediaRating::MatureAudiences => serializer.serialize_u32(3),
        }
    }
}

impl<'de> Deserialize<'de> for IqgMediaRating {
    fn deserialize<D>(deserializer: D) -> Result<IqgMediaRating, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(IqgMediaRating::AllAudiences),
            2 => Ok(IqgMediaRating::EveryoneOver12),
            3 => Ok(IqgMediaRating::MatureAudiences),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid IqgMediaRating value: {}",
                value
            ))),
        }
    }
}