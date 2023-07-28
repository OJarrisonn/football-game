use crate::data::game::player::Player;

pub struct Squad {
    pub name: String,
    pub players: [Player; 11]
}

impl Squad {
    pub fn new(name: &str, players: [Player; 11]) -> Squad {
        Squad { name: name.to_string(), players }
    }
}