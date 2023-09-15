use football_game::{data::{player::{Player, PlayerPosition}, attribute::{AttributeQuality, HomeFactor}, squad::Squad}, simulation::{self, calculate_goals}};

fn main() {
    let city_players = [
        Player::new(0, "Ederson", PlayerPosition::GK, AttributeQuality::HI),
        Player::new(0, "Walker", PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Ruben Dias", PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Stones", PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Ake", PlayerPosition::DF, AttributeQuality::HI),
        Player::new(0, "De Bruyne", PlayerPosition::MD, AttributeQuality::VH),
        Player::new(0, "Rodri", PlayerPosition::MD, AttributeQuality::VH),
        Player::new(0, "Foden", PlayerPosition::MD, AttributeQuality::HI),
        Player::new(0, "Bernardo", PlayerPosition::FW, AttributeQuality::VH),
        Player::new(0, "Haaland", PlayerPosition::FW, AttributeQuality::VH),
        Player::new(0, "Grealish", PlayerPosition::FW, AttributeQuality::HI),
    ];
    let city = Squad::new(0, "Manchester City", city_players);

    let barca_players = [
        Player::new(0, "Ter Stegen", PlayerPosition::GK, AttributeQuality::VH),
        Player::new(0, "Cancelo", PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Araújo", PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Kounde", PlayerPosition::DF, AttributeQuality::HI),
        Player::new(0, "Balde", PlayerPosition::DF, AttributeQuality::HI),
        Player::new(0, "Pedri", PlayerPosition::MD, AttributeQuality::HI),
        Player::new(0, "De Jong", PlayerPosition::MD, AttributeQuality::HI),
        Player::new(0, "Gundogan", PlayerPosition::MD, AttributeQuality::VH),
        Player::new(0, "Raphinha", PlayerPosition::FW, AttributeQuality::HI),
        Player::new(0, "Lewandowski", PlayerPosition::FW, AttributeQuality::VH),
        Player::new(0, "Ferran", PlayerPosition::FW, AttributeQuality::HI),
    ];

    let barca = Squad::new(1, "Barcelona", barca_players);
    let expected_goals_params: (f32, f32, f32) = (0.6, 1.5, 10.);
    let expected_goals: (f32, f32) = (simulation::expected_goals(&barca, &city, HomeFactor::new(AttributeQuality::OK), expected_goals_params), simulation::expected_goals(&city, &barca, HomeFactor::new(AttributeQuality::OK), expected_goals_params)); 
    let entropy: f32 = 1.25;

    println!("xG: {} x {}", expected_goals.0, expected_goals.1);

    let mut city_goals = 0;
    let mut barca_goals = 0;

    for _ in 0..100 {
        let barca_score = calculate_goals(expected_goals.0, entropy);
        let city_score = calculate_goals(expected_goals.1, entropy);
        
        println!("BAR {barca_score} x {city_score} CIT");

        barca_goals += barca_score;
        city_goals += city_score;
    }

    println!("Goal avg: BAR {} x {} CIT", (barca_goals as f32) / 100., (city_goals as f32) / 100.);
}

#[cfg(test)]
mod tests;