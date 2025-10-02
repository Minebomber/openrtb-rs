//! VAST 4.3 (Video Ad Serving Template) implementation
//!
//! This module provides a complete implementation of the IAB Tech Lab's VAST 4.3 specification,
//! which is used for serving video advertisements in a standardized XML format.

// Core modules
pub mod ad;
pub mod ad_parameters;
pub mod category;
pub mod common;
pub mod companion;
pub mod creative;
pub mod enums;
pub mod extensions;
pub mod icons;
pub mod inline;
pub mod linear;
pub mod media_file;
pub mod non_linear;
pub mod pricing;
pub mod tracking;
pub mod universal_ad_id;
pub mod vast;
pub mod verification;
pub mod viewable_impression;
pub mod wrapper;

// Re-export main types
pub use ad::*;
pub use ad_parameters::*;
pub use category::*;
pub use common::*;
pub use companion::*;
pub use creative::*;
pub use enums::*;
pub use extensions::*;
pub use icons::*;
pub use inline::*;
pub use linear::*;
pub use media_file::*;
pub use non_linear::*;
pub use pricing::*;
pub use tracking::*;
pub use universal_ad_id::*;
pub use vast::*;
pub use verification::*;
pub use viewable_impression::*;
pub use wrapper::*;
