use rand_distr::{Distribution, Normal};

use crate::data::{squad::Squad, player::PlayerPosition};

/// # expected_goals
///  Calculate the expected goals of given attack over a given defense
/// 
pub fn expected_goals(attacker: &Squad, defender: &Squad, home_factor: f32) -> f32 {
    let attack_quality: f32 = attacker.quality_of(|player| {*player.position() == PlayerPosition::FW || *player.position() == PlayerPosition::MD}).as_f32() + 2.; // Ranges from 0 to 4
    let defense_quality: f32 = defender.quality_of(|player| {*player.position() == PlayerPosition::DF || *player.position() == PlayerPosition::GK}).as_f32() + 2.; 

    let delta = attack_quality*attack_quality*home_factor - defense_quality*defense_quality*(1./home_factor); // [-16, 16] the delta quality difference
    println!("att: {attack_quality}, def: {defense_quality}, delta: {delta}");

    let a = 7. / (2. * 16. * 16.);
    let b = (1.25 + 7./2.) / 16.;
    let c = 1.75; 

    //0.008*delta*delta + 0.21875*delta + 1.5 - 0.048
    a*delta*delta + b*delta + c
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