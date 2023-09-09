use std::fmt::Display;

use serde_derive::{Serialize, Deserialize};

use crate::data::{util::attribute::AttributeQuality, GameObject};

use self::position::Position;

pub mod position;

#[derive(Serialize, Deserialize)]
pub struct Player {
    name: String,
    position: Position,
    rating: AttributeQuality
}

impl Player {
    pub fn new(name: &str, position: Position, rating: AttributeQuality) -> Player {
        Player { name: name.to_string(), position, rating }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
    pub fn rating(&self) -> &AttributeQuality {
        &self.rating
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}/{}] {}", self.rating, self.position, self.name)
    }
}

impl GameObject for Player {
    fn id(&self) -> String {
        self.name().to_lowercase().replace(" ", "_")
    }

    fn path(&self, root: &str) -> String {
        String::from(root) + &self.id() + ".yaml"
    }
}