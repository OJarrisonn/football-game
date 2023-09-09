use std::fs::{OpenOptions, File};

use football_game::data::player::{PlayerPosition, Player};
use football_game::data::attribute::AttributeQuality;
use serde_derive::{Serialize, Deserialize};

#[test]
fn sandbox() {
    let players = [Player::new(0, "Messi", PlayerPosition::FW, AttributeQuality::VH)];
}

#[test]
fn struct_to_yaml() {
    let messi = Player::new(0, "Lionel Messi", PlayerPosition::FW, AttributeQuality::VH);
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
        Player::new(0, "Ederson", PlayerPosition::GK, AttributeQuality::VH),
        Player::new(0, "Walker", PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Ruben Dias", PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Laporte", PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Ake", PlayerPosition::DF, AttributeQuality::HI),
        Player::new(0, "De Bruyne", PlayerPosition::MD, AttributeQuality::VH),
        Player::new(0, "Rodri", PlayerPosition::MD, AttributeQuality::VH),
        Player::new(0, "Foden", PlayerPosition::MD, AttributeQuality::HI),
        Player::new(0, "Bernardo", PlayerPosition::FW, AttributeQuality::VH),
        Player::new(0, "Haaland", PlayerPosition::FW, AttributeQuality::VH),
        Player::new(0, "Grealish", PlayerPosition::FW, AttributeQuality::HI),
    ];
    //let city = Squad::new("Manchester City", city_players);

    let barca_players = [
        Player::new(0, "Ter Stegen", PlayerPosition::GK, AttributeQuality::VH),
        Player::new(0, "Kounde", PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Araújo", PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Christensen", PlayerPosition::DF, AttributeQuality::HI),
        Player::new(0, "Balde", PlayerPosition::DF, AttributeQuality::HI),
        Player::new(0, "Pedri", PlayerPosition::MD, AttributeQuality::HI),
        Player::new(0, "De Jong", PlayerPosition::MD, AttributeQuality::HI),
        Player::new(0, "Gundogan", PlayerPosition::MD, AttributeQuality::VH),
        Player::new(0, "Raphinha", PlayerPosition::FW, AttributeQuality::HI),
        Player::new(0, "Lewandowski", PlayerPosition::FW, AttributeQuality::VH),
        Player::new(0, "Fati", PlayerPosition::FW, AttributeQuality::HI),
    ];
    //let barca = Squad::new("Barcelona", barca_players);

    //let city_file = OpenOptions::new().create(true).write(true).open("./data/squads/manchester_city.yaml").expect("Folder structure doesn't exists");
    //let barca_file = OpenOptions::new().create(true).write(true).open("./data/squads/barcelona.yaml").expect("Folder structure doesn't exists");

    //serde_yaml::to_writer(barca_file, &barca);
}

#[test]
fn serde_test() {
    let city_players = [
        Player::new(0, "Ederson",       PlayerPosition::GK, AttributeQuality::VH),
        Player::new(0, "Walker",        PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Ruben Dias",    PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Stones",        PlayerPosition::DF, AttributeQuality::HI),
        Player::new(0, "Ake",           PlayerPosition::DF, AttributeQuality::HI),
        Player::new(0, "De Bruyne",     PlayerPosition::MD, AttributeQuality::VH),
        Player::new(0, "Rodri",         PlayerPosition::MD, AttributeQuality::VH),
        Player::new(0, "Foden",         PlayerPosition::MD, AttributeQuality::HI),
        Player::new(0, "Bernardo",      PlayerPosition::FW, AttributeQuality::VH),
        Player::new(0, "Haaland",       PlayerPosition::FW, AttributeQuality::VH),
        Player::new(0, "Grealish",      PlayerPosition::FW, AttributeQuality::HI),
    ];
    //let city = Squad::new("Manchester City", city_players);

    let barca_players = [
        Player::new(0, "Ter Stegen",    PlayerPosition::GK, AttributeQuality::VH),
        Player::new(0, "Kounde",        PlayerPosition::DF, AttributeQuality::HI),
        Player::new(0, "Araújo",        PlayerPosition::DF, AttributeQuality::VH),
        Player::new(0, "Christensen",   PlayerPosition::DF, AttributeQuality::HI),
        Player::new(0, "Balde",         PlayerPosition::DF, AttributeQuality::HI),
        Player::new(0, "Pedri",         PlayerPosition::MD, AttributeQuality::HI),
        Player::new(0, "De Jong",       PlayerPosition::MD, AttributeQuality::HI),
        Player::new(0, "Gundogan",      PlayerPosition::MD, AttributeQuality::VH),
        Player::new(0, "Raphinha",      PlayerPosition::FW, AttributeQuality::HI),
        Player::new(0, "Lewandowski",   PlayerPosition::FW, AttributeQuality::VH),
        Player::new(0, "Fati",          PlayerPosition::FW, AttributeQuality::OK),
    ];
    //let barca = Squad::new("Barcelona", barca_players);


    /*match expimp::save_yaml("./data/squads/manchester_city.yaml", &city) {
        Ok(_) => println!("Squad file saved"),
        Err(e) => panic!("Couldn't save City squad. {}", e),
    }

    match expimp::load_yaml::<Squad>("./data/squads/manchester_city.yaml") {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("Couldn't load City squad, {}", e),
    }*/

}