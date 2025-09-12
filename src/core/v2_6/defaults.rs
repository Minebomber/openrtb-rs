use super::enums::*;

/// Default functions for fields with default values
pub fn default_at() -> Option<AuctionType> {
    Some(AuctionType::SecondPricePlus)
}

pub fn default_bidfloorcur() -> Option<String> {
    Some("USD".to_string())
}

pub fn default_skip_min() -> Option<u32> {
    Some(0)
}

pub fn default_skip_after() -> Option<u32> {
    Some(0)
}

pub fn default_boxingallowed() -> Option<BoxingAllowedFlag> {
    Some(BoxingAllowedFlag::Allowed)
}

pub fn default_cur() -> Option<String> {
    Some("USD".to_string())
}
