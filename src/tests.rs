use std::fs::{OpenOptions, File};

use serde_derive::{Serialize, Deserialize};

use crate::data::{util::attribute::AttributeQuality, game::{player::{Player, position::Position}, squad::Squad}};

#[test]
fn sandbox() {
    let players = [Player::new("Messi", Position::FW, AttributeQuality::Excelent)];
}

#[test]
fn struct_to_yaml() {
    let messi = Player::new("Lionel Messi", Position::FW, AttributeQuality::Excelent);
    let file = OpenOptions::new().write(true).create(true).open("messi.yaml").expect("File should exists");

    let yaml = serde_yaml::to_writer(file, &messi);
}

#[test]
fn yaml_to_struct() {
    let file = File::open("messi.yaml").unwrap();
    let messi: Player = serde_yaml::from_reader(file).unwrap();

    println!("{}", messi);
}

#[test]
fn creating_squad_file() {
    let city_players = [
        Player::new("Ederson", Position::GK, AttributeQuality::Excelent),
        Player::new("Walker", Position::DEF, AttributeQuality::Excelent),
        Player::new("Ruben Dias", Position::DEF, AttributeQuality::Excelent),
        Player::new("Laporte", Position::DEF, AttributeQuality::Excelent),
        Player::new("Ake", Position::DEF, AttributeQuality::Good),
        Player::new("De Bruyne", Position::MF, AttributeQuality::Excelent),
        Player::new("Rodri", Position::MF, AttributeQuality::Excelent),
        Player::new("Foden", Position::MF, AttributeQuality::Good),
        Player::new("Bernardo", Position::FW, AttributeQuality::Excelent),
        Player::new("Haaland", Position::FW, AttributeQuality::Excelent),
        Player::new("Grealish", Position::FW, AttributeQuality::Good),
    ];
    let city = Squad::new("Manchester City", city_players);

    let barca_players = [
        Player::new("Ter Stegen", Position::GK, AttributeQuality::Excelent),
        Player::new("Kounde", Position::DEF, AttributeQuality::Excelent),
        Player::new("Araújo", Position::DEF, AttributeQuality::Excelent),
        Player::new("Christensen", Position::DEF, AttributeQuality::Good),
        Player::new("Balde", Position::DEF, AttributeQuality::Good),
        Player::new("Pedri", Position::MF, AttributeQuality::Good),
        Player::new("De Jong", Position::MF, AttributeQuality::Good),
        Player::new("Gundogan", Position::MF, AttributeQuality::Excelent),
        Player::new("Raphinha", Position::FW, AttributeQuality::Good),
        Player::new("Lewandowski", Position::FW, AttributeQuality::Excelent),
        Player::new("Fati", Position::FW, AttributeQuality::Good),
    ];
    let barca = Squad::new("Barcelona", barca_players);

    let city_file = OpenOptions::new().create(true).write(true).open("./data/squads/manchester_city.yaml").expect("Folder structure doesn't exists");
    let barca_file = OpenOptions::new().create(true).write(true).open("./data/squads/barcelona.yaml").expect("Folder structure doesn't exists");

    serde_yaml::to_writer(barca_file, &barca);
}