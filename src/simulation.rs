use rand_distr::{Distribution, Normal};

use crate::data::{squad::Squad, player::PlayerPosition, attribute::HomeFactor};

/// # expected_goals
///  Calculate the expected goals of given attack over a given defense
/// 
pub fn expected_goals(attacker: &Squad, defender: &Squad, home_factor: HomeFactor, (mG, zG, MG): (f32, f32, f32)) -> f32 {
    let attack_quality: f32 = attacker.quality_of_as_f32(|player| {*player.position() == PlayerPosition::FW || *player.position() == PlayerPosition::MD}) + 2.; // Ranges from 0 to 4
    let defense_quality: f32 = defender.quality_of_as_f32(|player| {*player.position() == PlayerPosition::DF || *player.position() == PlayerPosition::GK}) + 2.; 

    let delta = attack_quality*attack_quality - defense_quality*defense_quality; // [-16, 16] the delta quality difference
    println!("att: {attack_quality}, def: {defense_quality}, delta: {delta}");

    let c = zG; 
    let a = ((MG + mG) - 2.*zG) / 512.;
    let b = (mG - zG - 256.*a) / (-16.);

    //0.008*delta*delta + 0.21875*delta + 1.5 - 0.048
    (a*delta*delta + b*delta + c) * home_factor.as_f32()
}

/// # calculate_goals
/// Calculate the amount of goals scored based on the expected goals and a random factor
/// 
pub fn calculate_goals(expected_goals: f32, entropy: f32) -> i32 {
    let normal_distribution = Normal::new(expected_goals, entropy).unwrap();
    let mut rng = rand::thread_rng();
    let v = normal_distribution.sample(&mut rng);
    return v.abs().round() as i32;
}