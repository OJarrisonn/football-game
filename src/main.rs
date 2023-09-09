use football_game::{data::{player::{Player, PlayerPosition}, attribute::AttributeQuality, squad::Squad}, simulation};

fn main() {
    let city_players = [
        Player::new(0, "Ederson", PlayerPosition::GK, AttributeQuality::VH),
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
        Player::new(0, "Fati", PlayerPosition::FW, AttributeQuality::HI),
    ];

    let barca = Squad::new(1, "Barcelona", barca_players);

    println!("{} x {}", simulation::expected_goals(&city, &barca, simulation::HOME_UNFAV), simulation::expected_goals(&barca, &city, simulation::HOME_FAV));
}

#[cfg(test)]
mod tests;