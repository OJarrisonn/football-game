use std::fmt::Display;

use serde_derive::{Serialize, Deserialize};

use super::attribute::AttributeQuality;


#[derive(Serialize, Deserialize, PartialEq)]
pub enum PlayerPosition {
    GK,
    DF,
    MD,
    FW
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    id: u32,
    name: String,
    position: PlayerPosition,
    quality: AttributeQuality
}

impl Player {
    pub fn new(id: u32, name: &str, position: PlayerPosition, quality: AttributeQuality) -> Self {
        Self {
            id, name: String::from(name), position, quality
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn position(&self) -> &PlayerPosition {
        &self.position
    }

    pub fn quality(&self) -> &AttributeQuality {
        &self.quality
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {} | {} ] {}", self.quality, self.position, self.name)
    }
}

impl Display for PlayerPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::FW => "FW",
            Self::MD => "MD",
            Self::DF => "DF",
            Self::GK => "GK"
        })
    }
}

impl Display for AttributeQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::VH => "VH",
            Self::HI => "HI",
            Self::OK => "OK",
            Self::LO => "LO",
            Self::VL => "VL"
        })
    }
}