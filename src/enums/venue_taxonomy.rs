use serde::{Deserialize, Serialize};

/// Venue Taxonomy
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum VenueTaxonomy {
    /// AdCom 1.0
    AdCom1 = 1,
}

