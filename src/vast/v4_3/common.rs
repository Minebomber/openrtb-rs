//! Common types and utilities used across VAST elements

use super::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Duration in HH:MM:SS format as specified in VAST
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    hard_xml::XmlWrite,
    hard_xml::XmlRead,
)]
#[xml(tag = "Duration")]
pub struct Duration {
    #[xml(text)]
    pub value: String,
}

impl Duration {
    /// Create a new Duration from HH:MM:SS string
    pub fn new(s: String) -> Self {
        Duration { value: s }
    }

    /// Parse duration into total seconds
    pub fn to_seconds(&self) -> Result<u32, super::VastError> {
        let parts: Vec<&str> = self.value.split(':').collect();
        if parts.len() != 3 {
            return Err(VastError::InvalidDuration(self.value.clone()));
        }

        let hours = parts[0]
            .parse::<u32>()
            .map_err(|_| VastError::InvalidDuration(self.value.clone()))?;
        let minutes = parts[1]
            .parse::<u32>()
            .map_err(|_| VastError::InvalidDuration(self.value.clone()))?;
        let seconds = parts[2]
            .parse::<u32>()
            .map_err(|_| VastError::InvalidDuration(self.value.clone()))?;

        Ok(hours * 3600 + minutes * 60 + seconds)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::str::FromStr for Duration {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Duration {
            value: s.to_string(),
        })
    }
}

// Duration doesn't need custom XmlRead/XmlWrite because hard_xml
// can derive them automatically when using #[xml(text)]
// We'll just implement Display and FromStr for attribute usage

/// MIME type for media files
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MimeType(pub String);

impl MimeType {
    pub fn new(s: String) -> Self {
        MimeType(s)
    }
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for MimeType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MimeType(s.to_string()))
    }
}

impl hard_xml::XmlWrite for MimeType {
    fn to_writer<W: std::io::Write>(
        &self,
        writer: &mut hard_xml::XmlWriter<W>,
    ) -> hard_xml::XmlResult<()> {
        writer.write_text(&self.0).map_err(hard_xml::XmlError::IO)
    }
}

impl<'a> hard_xml::XmlRead<'a> for MimeType {
    fn from_reader(reader: &mut hard_xml::XmlReader<'a>) -> hard_xml::XmlResult<Self> {
        let s = reader.read_text("MimeType")?;
        Ok(MimeType(s.to_string()))
    }
}

/// A URI/URL reference
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Uri(pub String);

impl Uri {
    pub fn new(s: String) -> Self {
        Uri(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Uri {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Uri(s.to_string()))
    }
}

impl hard_xml::XmlWrite for Uri {
    fn to_writer<W: std::io::Write>(
        &self,
        writer: &mut hard_xml::XmlWriter<W>,
    ) -> hard_xml::XmlResult<()> {
        writer.write_text(&self.0).map_err(hard_xml::XmlError::IO)
    }
}

impl<'a> hard_xml::XmlRead<'a> for Uri {
    fn from_reader(reader: &mut hard_xml::XmlReader<'a>) -> hard_xml::XmlResult<Self> {
        let s = reader.read_text("Uri")?;
        Ok(Uri(s.to_string()))
    }
}

/// Language code (e.g., "en", "es", "fr")
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Language(pub String);

impl Language {
    pub fn new(s: String) -> Self {
        Language(s)
    }
}

/// Represents a percentage value (0-100)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Percentage(pub f32);

impl Percentage {
    pub fn new(value: f32) -> Self {
        Percentage(value.clamp(0.0, 100.0))
    }
}

/// Ad ID type
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdId(pub String);

impl AdId {
    pub fn new(s: String) -> Self {
        AdId(s)
    }
}

/// Creative ID type
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreativeId(pub String);

impl CreativeId {
    pub fn new(s: String) -> Self {
        CreativeId(s)
    }
}

/// API Framework identifier
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiFramework(pub String);

impl ApiFramework {
    pub fn new(s: String) -> Self {
        ApiFramework(s)
    }
}

impl fmt::Display for ApiFramework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ApiFramework {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ApiFramework(s.to_string()))
    }
}

impl hard_xml::XmlWrite for ApiFramework {
    fn to_writer<W: std::io::Write>(
        &self,
        writer: &mut hard_xml::XmlWriter<W>,
    ) -> hard_xml::XmlResult<()> {
        writer.write_text(&self.0).map_err(hard_xml::XmlError::IO)
    }
}

impl<'a> hard_xml::XmlRead<'a> for ApiFramework {
    fn from_reader(reader: &mut hard_xml::XmlReader<'a>) -> hard_xml::XmlResult<Self> {
        let s = reader.read_text("ApiFramework")?;
        Ok(ApiFramework(s.to_string()))
    }
}
