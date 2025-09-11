use serde::{Deserialize, Serialize};

/// Auction Type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum AuctionType {
    /// First Price
    FirstPrice = 1,
    /// Second Price Plus
    SecondPricePlus = 2,
}

