use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Volume Normalization Modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VolumeNormalizationMode {
    /// None
    None,
    /// Ad Volume Average Normalized to Content
    AdVolumeAverageNormalizedToContent,
    /// Ad Volume Peak Normalized to Content
    AdVolumePeakNormalizedToContent,
    /// Ad Loudness Normalized to Content
    AdLoudnessNormalizedToContent,
    /// Custom Volume Normalization
    CustomVolumeNormalization,
}

impl Serialize for VolumeNormalizationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            VolumeNormalizationMode::None => serializer.serialize_u32(0),
            VolumeNormalizationMode::AdVolumeAverageNormalizedToContent => serializer.serialize_u32(1),
            VolumeNormalizationMode::AdVolumePeakNormalizedToContent => serializer.serialize_u32(2),
            VolumeNormalizationMode::AdLoudnessNormalizedToContent => serializer.serialize_u32(3),
            VolumeNormalizationMode::CustomVolumeNormalization => serializer.serialize_u32(4),
        }
    }
}

impl<'de> Deserialize<'de> for VolumeNormalizationMode {
    fn deserialize<D>(deserializer: D) -> Result<VolumeNormalizationMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(VolumeNormalizationMode::None),
            1 => Ok(VolumeNormalizationMode::AdVolumeAverageNormalizedToContent),
            2 => Ok(VolumeNormalizationMode::AdVolumePeakNormalizedToContent),
            3 => Ok(VolumeNormalizationMode::AdLoudnessNormalizedToContent),
            4 => Ok(VolumeNormalizationMode::CustomVolumeNormalization),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid VolumeNormalizationMode value: {}",
                value
            ))),
        }
    }
}