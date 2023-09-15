use serde_derive::{Serialize, Deserialize};

use super::{player::Player, attribute::AttributeQuality};

#[derive(Serialize, Deserialize)]
pub struct Squad {
    id: u32,
    name: String,
    players: [Player; 11]
}

impl Squad {
    pub fn new(id: u32, name: &str, players: [Player; 11]) -> Self {
        Self { id, name: String::from(name), players}
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn players(&self) -> &[Player; 11] {
        &self.players
    }

    pub fn quality_of<P>(&self, filter: P) -> AttributeQuality
    where P: Fn(&&Player) -> bool {
        let mut quality: f32 = 0.0;
        let mut count: i32 = 0;

        for player in self.players.iter().filter(filter) {
            quality += player.quality().as_f32();
            count += 1;
        }

        AttributeQuality::from(quality / (count as f32))
    }

    pub fn quality_of_as_f32<P>(&self, filter: P) -> f32
    where P: Fn(&&Player) -> bool {
        let mut quality: f32 = 0.0;
        let mut count: i32 = 0;

        for player in self.players.iter().filter(filter) {
            quality += player.quality().as_f32();
            count += 1;
        }

        (quality / (count as f32))
    }
}

