use diysh::{shell::Shell, commands::{definition::CommandDefinition, argument::ArgType}};

pub struct Config {
    // Shell
    shell: Shell,

    // Storage paths
    root_directory: String,
    folder_structure: Vec<String>
}

impl Config {
    pub fn new(root: &str) -> Self {
        let root = root.trim();
        let root = if !root.ends_with('/') {
            format!("{}/", root)
        } else {
            root.to_string()
        };

        Self { 
            shell: Shell::new(), 
            root_directory: root.to_string(), 
            folder_structure: vec![
                root.clone() + "config/",
                root.clone() + "data/",
                root.clone() + "log/"
            ] 
        }
    }

    pub fn setup(&mut self) {
        self.shell
            .set_prompt("~>>")
            .register_help()
            .register_command(
                CommandDefinition::new("squad1")
                .add_arg(ArgType::Str)
                .set_callback(|_shell, args| {
                    let squad_id = args[0].get_str().unwrap();
                    
                }).build()    
            );
    }
}