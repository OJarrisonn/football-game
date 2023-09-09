use crate::data::{squad::Squad, player::PlayerPosition};

pub const HOME_NEUTRAL: f32 = 1.;
pub const HOME_FAV: f32 = 1.05;
pub const HOME_SUP_FAV: f32 = 1.1;
pub const HOME_UNFAV: f32 = 0.95;
pub const HOME_SUP_UNFAV: f32 = 0.9;

/// # Calculate the expected goals of given attack over a given defense
/// 
pub fn expected_goals(attacker: &Squad, defender: &Squad, home_factor: f32) -> f32 {
    let attack_quality = attacker.quality_of(|player| {*player.position() == PlayerPosition::FW || *player.position() == PlayerPosition::MD}).as_float() + 2.; // Ranges from 0 to 4
    let defense_quality = defender.quality_of(|player| {*player.position() == PlayerPosition::DF || *player.position() == PlayerPosition::GK}).as_float() + 2.; 

    let delta = attack_quality*attack_quality*home_factor - defense_quality*defense_quality*(1./home_factor); // [-16, 16] the delta quality difference

    0.008*delta*delta + 0.21875*delta + 1.5 - 0.048
}