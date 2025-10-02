//! Creative type enumeration

use super::super::*;
use hard_xml::{XmlRead, XmlWrite};

/// The type of creative content
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
pub enum CreativeType {
    /// Linear video or audio creative
    #[xml(tag = "Linear")]
    Linear(Linear),

    /// Non-linear overlay creative
    #[xml(tag = "NonLinearAds")]
    NonLinearAds(NonLinearAds),

    /// Companion banner creative
    #[xml(tag = "CompanionAds")]
    CompanionAds(CompanionAds),
}
