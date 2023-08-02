use std::fmt::Display;

use serde_derive::{Serialize, Deserialize};

use crate::data::game::player::Player;

#[derive(Serialize, Deserialize)]
pub struct Squad {
    name: String,
    players: [Player; 11]
}

impl Squad {
    pub fn new(name: &str, players: [Player; 11]) -> Squad {
        Squad { name: name.to_string(), players }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn players(&self) -> &[Player; 11] {
        &self.players
    }
}

impl Display for Squad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Squad: {} -> {}", self.name(),
            {
                let mut players_list = String::new();

                for player in self.players() {
                    players_list.push_str(player.name());
                    players_list.push_str(", ");
                }

                players_list
            }
        )
    }
}