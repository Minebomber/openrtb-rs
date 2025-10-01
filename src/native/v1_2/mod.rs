//! OpenRTB Native 1.2 Specification Objects
//!
//! This module contains all the data structures defined in the OpenRTB Native 1.2 specification.
//! The specification defines a sub-protocol of OpenRTB to allow for the delivery of native
//! advertising formats.

pub mod enums;
pub mod request;
pub mod response;

// Re-export all types for easy access
pub use enums::*;
pub use request::*;
pub use response::*;

