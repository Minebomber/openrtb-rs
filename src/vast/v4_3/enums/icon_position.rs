//! Icon position enumeration

/// Icon position (can be pixel value or left/right/top/bottom)
#[derive(Debug, Clone, PartialEq)]
pub enum IconPosition {
    /// Left position
    Left,
    /// Right position
    Right,
    /// Top position
    Top,
    /// Bottom position
    Bottom,
    /// Numeric pixel position
    Pixels(u32),
}

impl std::fmt::Display for IconPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IconPosition::Left => write!(f, "left"),
            IconPosition::Right => write!(f, "right"),
            IconPosition::Top => write!(f, "top"),
            IconPosition::Bottom => write!(f, "bottom"),
            IconPosition::Pixels(n) => write!(f, "{}", n),
        }
    }
}

impl std::str::FromStr for IconPosition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(IconPosition::Left),
            "right" => Ok(IconPosition::Right),
            "top" => Ok(IconPosition::Top),
            "bottom" => Ok(IconPosition::Bottom),
            _ => s
                .parse::<u32>()
                .map(IconPosition::Pixels)
                .map_err(|_| format!("Invalid icon position: {}", s)),
        }
    }
}
