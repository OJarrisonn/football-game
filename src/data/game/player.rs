use std::fmt::Display;

use serde_derive::{Serialize, Deserialize};

use crate::data::util::attribute::AttributeQuality;

use self::position::Position;

pub mod position;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub position: Position,
    pub rating: AttributeQuality
}

impl Player {
    pub fn new(name: &str, position: Position, rating: AttributeQuality) -> Player {
        Player { name: name.to_string(), position, rating }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}/{}] {}", self.rating, self.position, self.name)
    }
}