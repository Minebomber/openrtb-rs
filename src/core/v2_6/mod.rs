//! OpenRTB 2.6 Specification Objects
//!
//! This module contains all the data structures defined in the OpenRTB 2.6 specification.

// Bid Request objects
pub mod app;
pub mod audio;
pub mod banner;
pub mod bid_request;
pub mod brand;
pub mod channel;
pub mod content;
pub mod data;
pub mod deal;
pub mod device;
pub mod dooh;
pub mod format;
pub mod geo;
pub mod impression;
pub mod metric;
pub mod native;
pub mod network;
pub mod pmp;
pub mod producer;
pub mod publisher;
pub mod regs;
pub mod segment;
pub mod site;
pub mod source;
pub mod user;
pub mod user_agent;
pub mod video;

// Bid Response objects
pub mod bid;
pub mod bid_response;
pub mod seat_bid;

// Re-export all structs for easy access
pub use app::*;
pub use audio::*;
pub use banner::*;
pub use bid::*;
pub use bid_request::*;
pub use bid_response::*;
pub use brand::*;
pub use channel::*;
pub use content::*;
pub use data::*;
pub use deal::*;
pub use device::*;
pub use dooh::*;
pub use format::*;
pub use geo::*;
pub use impression::*;
pub use metric::*;
pub use native::*;
pub use network::*;
pub use pmp::*;
pub use producer::*;
pub use publisher::*;
pub use regs::*;
pub use seat_bid::*;
pub use segment::*;
pub use site::*;
pub use source::*;
pub use user::*;
pub use user_agent::*;
pub use video::*;

pub mod enums;
