use serde::{Deserialize, Serialize};

/// Content Taxonomy
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum ContentTaxonomy {
    /// IAB Content Category Taxonomy 1.0
    IabContentCategory1 = 1,
    /// IAB Content Category Taxonomy 2.0
    IabContentCategory2 = 2,
}

