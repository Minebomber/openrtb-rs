use serde::{Deserialize, Serialize};

/// Loss Reason Codes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum LossReasonCode {
    /// Bid Won
    BidWon = 0,
    /// Internal Error
    InternalError = 1,
    /// Impression Opportunity Expired
    ImpressionOpportunityExpired = 2,
    /// Invalid Bid Response
    InvalidBidResponse = 3,
    /// Invalid Deal ID
    InvalidDealId = 4,
    /// Invalid Auction ID
    InvalidAuctionId = 5,
    /// Invalid (i.e., malformed) Advertiser Domain
    InvalidAdvertiserDomain = 6,
    /// Missing Markup
    MissingMarkup = 7,
    /// Missing Creative ID
    MissingCreativeId = 8,
    /// Missing Bid Price
    MissingBidPrice = 9,
    /// Missing Minimum Creative Approval Data
    MissingMinimumCreativeApprovalData = 10,
    /// Bid was Below Auction Floor
    BidBelowAuctionFloor = 100,
    /// Bid was Below Deal Floor
    BidBelowDealFloor = 101,
    /// Lost to Higher Bid
    LostToHigherBid = 102,
    /// Lost to a Bid for a PMP Deal
    LostToPmpDeal = 103,
    /// Buyer Seat Blocked
    BuyerSeatBlocked = 200,
    /// Creative Filtered General
    CreativeFilteredGeneral = 201,
    /// Creative Filtered Pending Processing by Exchange
    CreativeFilteredPendingProcessing = 202,
    /// Creative Filtered Disapproved by Exchange
    CreativeFilteredDisapproved = 203,
    /// Creative Filtered Size Not Allowed
    CreativeFilteredSizeNotAllowed = 204,
    /// Creative Filtered Incorrect Creative Format
    CreativeFilteredIncorrectFormat = 205,
    /// Creative Filtered Advertiser Exclusions
    CreativeFilteredAdvertiserExclusions = 206,
    /// Creative Filtered App Bundle Exclusions
    CreativeFilteredAppBundleExclusions = 207,
    /// Creative Filtered Not Secure
    CreativeFilteredNotSecure = 208,
    /// Creative Filtered Language Exclusions
    CreativeFilteredLanguageExclusions = 209,
    /// Creative Filtered Category Exclusions
    CreativeFilteredCategoryExclusions = 210,
    /// Creative Filtered Creative Attribute Restrictions
    CreativeFilteredAttributeRestrictions = 211,
    /// Creative Filtered Ad Type Exclusions
    CreativeFilteredAdTypeExclusions = 212,
    /// Creative Filtered Animation Too Long
    CreativeFilteredAnimationTooLong = 213,
    /// Creative Filtered Not Allowed in PMP Deal
    CreativeFilteredNotAllowedInPmpDeal = 214,
}

