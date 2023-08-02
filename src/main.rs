use std::{path::Path, fs};

use diysh::{shell::Shell, inout::log::LogLevel};

pub const DATA_PATH: &str = "./data/";
const LOG_PATH: &str = "./log/";

fn main() {
    let mut shell = Shell::new();

    setup(&mut shell);

    println!("Football Game a0");

    loop {
        shell.read_and_run();
    }
}

fn setup(shell: &mut Shell) {
    shell
        .set_log_directory(LOG_PATH)
        .register_help()
        .register_history()
        .register_exit()
        .set_prompt("~$ ")
        .set_sparse(true);


    let data_path = Path::new(DATA_PATH);

    if !data_path.is_dir() {
        match fs::create_dir(data_path) {
            Ok(_) => {
                shell.log(LogLevel::INFO,"Data folder created");
                let squads_path = String::from(DATA_PATH) + "squads/";
                match fs::create_dir(Path::new(&squads_path)) {
                    Ok(_) => shell.log(LogLevel::INFO, "Squads folder created"),
                    Err(e) => shell.log(LogLevel::ERROR, &format!("Squads folder couldn't be created, due to {}", e)),
                }
            },
            Err(e) => shell.log(LogLevel::ERROR, &format!("Squads folder couldn't be created, due to {}", e)),
        }
    }
}