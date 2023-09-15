use std::fmt::Display;

use serde_derive::{Serialize, Deserialize};

use super::attribute::AttributeQuality;


#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
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
    quality: AttributeQuality,
    player_attributes: PlayerAttributes
}

pub struct PlayerBuilder {
    id: u32,
    name: String,
    position: PlayerPosition,
    quality: AttributeQuality,
    player_attributes: PlayerAttributes
}


impl Player {
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


impl PlayerBuilder {
    pub fn new(id: u32, name: String) -> PlayerBuilder {
        PlayerBuilder {
            id,
            name,
            position: PlayerPosition::GK,
            quality: AttributeQuality::OK,
            player_attributes: PlayerAttributesBuilder::new().build(),
        }
    }

    pub fn position(&mut self, position: PlayerPosition) -> &mut PlayerBuilder {
        self.position = position;
        self
    }

    pub fn quality(&mut self, quality: AttributeQuality) -> &mut PlayerBuilder {
        self.quality = quality;
        self
    }

    pub fn player_attributes(&mut self, player_attributes: PlayerAttributes) -> &mut PlayerBuilder {
        self.player_attributes = player_attributes;
        self
    }

    pub fn build(&self) -> Player {
        Player {
            id: self.id,
            name: self.name.clone(),
            position: self.position,
            quality: self.quality,
            player_attributes: self.player_attributes,
        }
    }
}


/// # Player Attributes
/// Used to determine the hability of each player using an AttributeQuality
/// 
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct PlayerAttributes {
    // Physical =================

    strength: AttributeQuality,
    speed: AttributeQuality,
    agility: AttributeQuality,
    impulsion: AttributeQuality,
    elasticity: AttributeQuality,

    // Technical ================

    dribbling: AttributeQuality,
    ball_control: AttributeQuality,
    passing: AttributeQuality,
    long_passing: AttributeQuality,
    shooting: AttributeQuality,
    long_shooting: AttributeQuality,
    positioning: AttributeQuality,
    tackle: AttributeQuality

}

pub struct PlayerAttributesBuilder {
    strength: Option<AttributeQuality>,
    speed: Option<AttributeQuality>,
    agility: Option<AttributeQuality>,
    impulsion: Option<AttributeQuality>,
    elasticity: Option<AttributeQuality>,
    dribbling: Option<AttributeQuality>,
    ball_control: Option<AttributeQuality>,
    passing: Option<AttributeQuality>,
    long_passing: Option<AttributeQuality>,
    shooting: Option<AttributeQuality>,
    long_shooting: Option<AttributeQuality>,
    positioning: Option<AttributeQuality>,
    tackle: Option<AttributeQuality>,
}

impl PlayerAttributesBuilder {
    pub fn new() -> PlayerAttributesBuilder {
        PlayerAttributesBuilder {
            strength: None,
            speed: None,
            agility: None,
            impulsion: None,
            elasticity: None,
            dribbling: None,
            ball_control: None,
            passing: None,
            long_passing: None,
            shooting: None,
            long_shooting: None,
            positioning: None,
            tackle: None,
        }
    }

    pub fn strength(mut self, strength: AttributeQuality) -> Self {
        self.strength = Some(strength);
        self
    }

    pub fn speed(mut self, speed: AttributeQuality) -> Self {
        self.speed = Some(speed);
        self
    }

    pub fn agility(mut self, agility: AttributeQuality) -> Self {
        self.agility = Some(agility);
        self
    }

    pub fn impulsion(mut self, impulsion: AttributeQuality) -> Self {
        self.impulsion = Some(impulsion);
        self
    }

    pub fn elasticity(mut self, elasticity: AttributeQuality) -> Self {
        self.elasticity = Some(elasticity);
        self
    }
    pub fn dribbling(mut self, dribbling: AttributeQuality) -> Self {
        self.dribbling = Some(dribbling);
        self
    }
    pub fn ball_control(mut self, ball_control: AttributeQuality) -> Self {
        self.ball_control = Some(ball_control);
        self
    }
    pub fn passing(mut self, passing: AttributeQuality) -> Self {
        self.passing = Some(passing);
        self
    }
    pub fn long_passing(mut self, long_passing: AttributeQuality) -> Self {
        self.long_passing = Some(long_passing);
        self
    }
    pub fn shooting(mut self, shooting: AttributeQuality) -> Self {
        self.shooting = Some(shooting);
        self
    }
    pub fn long_shooting(mut self, long_shooting: AttributeQuality) -> Self {
        self.long_shooting = Some(long_shooting);
        self
    }
    pub fn positioning(mut self, positioning: AttributeQuality) -> Self {
        self.positioning = Some(positioning);
        self
    }

    pub fn tackle(mut self, tackle: AttributeQuality) -> Self {
        self.tackle = Some(tackle);
        self
    }

    pub fn build(self) -> PlayerAttributes {
        PlayerAttributes {
            strength:           self.strength.unwrap_or(AttributeQuality::OK),
            speed:              self.speed.unwrap_or(AttributeQuality::OK),
            agility:            self.agility.unwrap_or(AttributeQuality::OK),
            impulsion:          self.impulsion.unwrap_or(AttributeQuality::OK),
            elasticity:         self.elasticity.unwrap_or(AttributeQuality::OK),
            dribbling:          self.dribbling.unwrap_or(AttributeQuality::OK),
            ball_control:       self.ball_control.unwrap_or(AttributeQuality::OK),
            passing:            self.passing.unwrap_or(AttributeQuality::OK),
            long_passing:       self.long_passing.unwrap_or(AttributeQuality::OK),
            shooting:           self.shooting.unwrap_or(AttributeQuality::OK),
            long_shooting:      self.long_shooting.unwrap_or(AttributeQuality::OK),
            positioning:        self.positioning.unwrap_or(AttributeQuality::OK),
            tackle:             self.tackle.unwrap_or(AttributeQuality::OK),
        }
    }
}