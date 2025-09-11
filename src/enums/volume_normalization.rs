use serde::{Deserialize, Serialize};

/// Volume Normalization Modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum VolumeNormalizationMode {
    /// None
    None = 0,
    /// Ad Volume Average Normalized to Content
    AdVolumeAverageNormalizedToContent = 1,
    /// Ad Volume Peak Normalized to Content
    AdVolumePeakNormalizedToContent = 2,
    /// Ad Loudness Normalized to Content
    AdLoudnessNormalizedToContent = 3,
    /// Custom Volume Normalization
    CustomVolumeNormalization = 4,
}

