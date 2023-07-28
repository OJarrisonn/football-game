use std::fmt::Display;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum AttributeQuality {
    Awful,
    Bad,
    Average,
    Good,
    Excelent
}

impl Display for AttributeQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeQuality::Awful => write!(f, "AWF"),
            AttributeQuality::Bad => write!(f, "BAD"),
            AttributeQuality::Average => write!(f, "AVG"),
            AttributeQuality::Good => write!(f, "GOOD"),
            AttributeQuality::Excelent => write!(f, "EXC")
        }
    }
}