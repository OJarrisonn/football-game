use std::fmt::Display;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Position {
    GK,
    DEF,
    MF,
    FW
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::GK => write!(f, "GK"),
            Position::DEF => write!(f, "DEF"),
            Position::MF => write!(f, "MF"),
            Position::FW => write!(f, "FW")
        }
    }
}