//! Delivery method for media files

/// Delivery method for media files
#[derive(Debug, Clone, PartialEq)]
pub enum DeliveryType {
    /// Progressive download
    Progressive,
    /// Streaming
    Streaming,
}

impl std::fmt::Display for DeliveryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeliveryType::Progressive => write!(f, "progressive"),
            DeliveryType::Streaming => write!(f, "streaming"),
        }
    }
}

impl std::str::FromStr for DeliveryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "progressive" => Ok(DeliveryType::Progressive),
            "streaming" => Ok(DeliveryType::Streaming),
            _ => Err(format!("Unknown delivery type: {}", s)),
        }
    }
}
