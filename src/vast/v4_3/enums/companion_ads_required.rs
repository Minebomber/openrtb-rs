//! Companion ad requirement options

/// Companion ad requirement options
#[derive(Debug, Clone, PartialEq)]
pub enum CompanionAdsRequired {
    /// All companion ads must be displayed
    All,
    /// At least one companion ad must be displayed
    Any,
    /// No requirement for companion ad display
    None,
}

impl std::fmt::Display for CompanionAdsRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompanionAdsRequired::All => write!(f, "all"),
            CompanionAdsRequired::Any => write!(f, "any"),
            CompanionAdsRequired::None => write!(f, "none"),
        }
    }
}

impl std::str::FromStr for CompanionAdsRequired {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(CompanionAdsRequired::All),
            "any" => Ok(CompanionAdsRequired::Any),
            "none" => Ok(CompanionAdsRequired::None),
            _ => Err(format!("Unknown required value: {}", s)),
        }
    }
}
