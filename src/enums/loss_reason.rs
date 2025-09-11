use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Loss Reason Codes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LossReasonCode {
    /// Bid Won
    BidWon,
    /// Internal Error
    InternalError,
    /// Impression Opportunity Expired
    ImpressionOpportunityExpired,
    /// Invalid Bid Response
    InvalidBidResponse,
    /// Invalid Deal ID
    InvalidDealId,
    /// Invalid Auction ID
    InvalidAuctionId,
    /// Invalid (i.e., malformed) Advertiser Domain
    InvalidAdvertiserDomain,
    /// Missing Markup
    MissingMarkup,
    /// Missing Creative ID
    MissingCreativeId,
    /// Missing Bid Price
    MissingBidPrice,
    /// Missing Minimum Creative Approval Data
    MissingMinimumCreativeApprovalData,
    /// Bid was Below Auction Floor
    BidBelowAuctionFloor,
    /// Bid was Below Deal Floor
    BidBelowDealFloor,
    /// Lost to Higher Bid
    LostToHigherBid,
    /// Lost to a Bid for a PMP Deal
    LostToPmpDeal,
    /// Buyer Seat Blocked
    BuyerSeatBlocked,
    /// Creative Filtered General
    CreativeFilteredGeneral,
    /// Creative Filtered Pending Processing by Exchange
    CreativeFilteredPendingProcessing,
    /// Creative Filtered Disapproved by Exchange
    CreativeFilteredDisapproved,
    /// Creative Filtered Size Not Allowed
    CreativeFilteredSizeNotAllowed,
    /// Creative Filtered Incorrect Creative Format
    CreativeFilteredIncorrectFormat,
    /// Creative Filtered Advertiser Exclusions
    CreativeFilteredAdvertiserExclusions,
    /// Creative Filtered App Bundle Exclusions
    CreativeFilteredAppBundleExclusions,
    /// Creative Filtered Not Secure
    CreativeFilteredNotSecure,
    /// Creative Filtered Language Exclusions
    CreativeFilteredLanguageExclusions,
    /// Creative Filtered Category Exclusions
    CreativeFilteredCategoryExclusions,
    /// Creative Filtered Creative Attribute Restrictions
    CreativeFilteredAttributeRestrictions,
    /// Creative Filtered Ad Type Exclusions
    CreativeFilteredAdTypeExclusions,
    /// Creative Filtered Animation Too Long
    CreativeFilteredAnimationTooLong,
    /// Creative Filtered Not Allowed in PMP Deal
    CreativeFilteredNotAllowedInPmpDeal,
}

impl Serialize for LossReasonCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            LossReasonCode::BidWon => serializer.serialize_u32(0),
            LossReasonCode::InternalError => serializer.serialize_u32(1),
            LossReasonCode::ImpressionOpportunityExpired => serializer.serialize_u32(2),
            LossReasonCode::InvalidBidResponse => serializer.serialize_u32(3),
            LossReasonCode::InvalidDealId => serializer.serialize_u32(4),
            LossReasonCode::InvalidAuctionId => serializer.serialize_u32(5),
            LossReasonCode::InvalidAdvertiserDomain => serializer.serialize_u32(6),
            LossReasonCode::MissingMarkup => serializer.serialize_u32(7),
            LossReasonCode::MissingCreativeId => serializer.serialize_u32(8),
            LossReasonCode::MissingBidPrice => serializer.serialize_u32(9),
            LossReasonCode::MissingMinimumCreativeApprovalData => serializer.serialize_u32(10),
            LossReasonCode::BidBelowAuctionFloor => serializer.serialize_u32(100),
            LossReasonCode::BidBelowDealFloor => serializer.serialize_u32(101),
            LossReasonCode::LostToHigherBid => serializer.serialize_u32(102),
            LossReasonCode::LostToPmpDeal => serializer.serialize_u32(103),
            LossReasonCode::BuyerSeatBlocked => serializer.serialize_u32(200),
            LossReasonCode::CreativeFilteredGeneral => serializer.serialize_u32(201),
            LossReasonCode::CreativeFilteredPendingProcessing => serializer.serialize_u32(202),
            LossReasonCode::CreativeFilteredDisapproved => serializer.serialize_u32(203),
            LossReasonCode::CreativeFilteredSizeNotAllowed => serializer.serialize_u32(204),
            LossReasonCode::CreativeFilteredIncorrectFormat => serializer.serialize_u32(205),
            LossReasonCode::CreativeFilteredAdvertiserExclusions => serializer.serialize_u32(206),
            LossReasonCode::CreativeFilteredAppBundleExclusions => serializer.serialize_u32(207),
            LossReasonCode::CreativeFilteredNotSecure => serializer.serialize_u32(208),
            LossReasonCode::CreativeFilteredLanguageExclusions => serializer.serialize_u32(209),
            LossReasonCode::CreativeFilteredCategoryExclusions => serializer.serialize_u32(210),
            LossReasonCode::CreativeFilteredAttributeRestrictions => serializer.serialize_u32(211),
            LossReasonCode::CreativeFilteredAdTypeExclusions => serializer.serialize_u32(212),
            LossReasonCode::CreativeFilteredAnimationTooLong => serializer.serialize_u32(213),
            LossReasonCode::CreativeFilteredNotAllowedInPmpDeal => serializer.serialize_u32(214),
        }
    }
}

impl<'de> Deserialize<'de> for LossReasonCode {
    fn deserialize<D>(deserializer: D) -> Result<LossReasonCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(LossReasonCode::BidWon),
            1 => Ok(LossReasonCode::InternalError),
            2 => Ok(LossReasonCode::ImpressionOpportunityExpired),
            3 => Ok(LossReasonCode::InvalidBidResponse),
            4 => Ok(LossReasonCode::InvalidDealId),
            5 => Ok(LossReasonCode::InvalidAuctionId),
            6 => Ok(LossReasonCode::InvalidAdvertiserDomain),
            7 => Ok(LossReasonCode::MissingMarkup),
            8 => Ok(LossReasonCode::MissingCreativeId),
            9 => Ok(LossReasonCode::MissingBidPrice),
            10 => Ok(LossReasonCode::MissingMinimumCreativeApprovalData),
            100 => Ok(LossReasonCode::BidBelowAuctionFloor),
            101 => Ok(LossReasonCode::BidBelowDealFloor),
            102 => Ok(LossReasonCode::LostToHigherBid),
            103 => Ok(LossReasonCode::LostToPmpDeal),
            200 => Ok(LossReasonCode::BuyerSeatBlocked),
            201 => Ok(LossReasonCode::CreativeFilteredGeneral),
            202 => Ok(LossReasonCode::CreativeFilteredPendingProcessing),
            203 => Ok(LossReasonCode::CreativeFilteredDisapproved),
            204 => Ok(LossReasonCode::CreativeFilteredSizeNotAllowed),
            205 => Ok(LossReasonCode::CreativeFilteredIncorrectFormat),
            206 => Ok(LossReasonCode::CreativeFilteredAdvertiserExclusions),
            207 => Ok(LossReasonCode::CreativeFilteredAppBundleExclusions),
            208 => Ok(LossReasonCode::CreativeFilteredNotSecure),
            209 => Ok(LossReasonCode::CreativeFilteredLanguageExclusions),
            210 => Ok(LossReasonCode::CreativeFilteredCategoryExclusions),
            211 => Ok(LossReasonCode::CreativeFilteredAttributeRestrictions),
            212 => Ok(LossReasonCode::CreativeFilteredAdTypeExclusions),
            213 => Ok(LossReasonCode::CreativeFilteredAnimationTooLong),
            214 => Ok(LossReasonCode::CreativeFilteredNotAllowedInPmpDeal),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid LossReasonCode value: {}",
                value
            ))),
        }
    }
}