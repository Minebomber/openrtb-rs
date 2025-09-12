//! OpenRTB 2.6 Enumerated Values
//!
//! This module contains all the enumerated types defined in the OpenRTB 2.6 specification.

pub mod ad_position;
pub mod api_frameworks;
pub mod auction_type;
pub mod banner_ad_type;
pub mod companion_type;
pub mod connection_type;
pub mod content_context;
pub mod content_delivery;
pub mod content_taxonomy;
pub mod creative_attributes;
pub mod device_type;
pub mod expandable_direction;
pub mod feed_type;
pub mod location_service;
pub mod location_type;
pub mod loss_reason;
pub mod no_bid_reason;
pub mod playback_cessation;
pub mod playback_method;
pub mod production_quality;
pub mod qag_media_rating;
pub mod start_delay;
pub mod user_agent_source;
pub mod venue_taxonomy;
pub mod video_linearity;
pub mod video_placement;
pub mod video_protocols;
pub mod volume_normalization;

// New enums for OpenRTB 2.6
pub mod flags;

// Re-export all enums for easy access
pub use ad_position::*;
pub use api_frameworks::*;
pub use auction_type::*;
pub use banner_ad_type::*;
pub use companion_type::*;
pub use connection_type::*;
pub use content_context::*;
pub use content_delivery::*;
pub use content_taxonomy::*;
pub use creative_attributes::*;
pub use device_type::*;
pub use expandable_direction::*;
pub use feed_type::*;
pub use flags::*;
pub use location_service::*;
pub use location_type::*;
pub use loss_reason::*;
pub use no_bid_reason::*;
pub use playback_cessation::*;
pub use playback_method::*;
pub use production_quality::*;
pub use qag_media_rating::*;
pub use start_delay::*;
pub use user_agent_source::*;
pub use venue_taxonomy::*;
pub use video_linearity::*;
pub use video_placement::*;
pub use video_protocols::*;
pub use volume_normalization::*;

