use std::fs::{OpenOptions, File};

use serde_derive::{Serialize, Deserialize};

use crate::data::{util::attribute::AttributeQuality, game::player::{Player, position::Position}};

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