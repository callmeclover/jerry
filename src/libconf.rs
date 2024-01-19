//mod libconf;
//use libconf::*;
use serde::*;
use std::{ fs, path::Path, any::Any};
use toml::{de::Error, from_str, to_string_pretty};
use ansi_term::Color;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    basic: Basic,
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    extra: Extra,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Basic {
    #[allow(dead_code)]
    use_mouse: bool,
    #[allow(dead_code)]
    use_keyboard: bool,
    #[allow(dead_code)]
    use_controller: bool,
    #[allow(dead_code)]
    do_screenshots: bool,
    #[allow(dead_code)]
    do_tts: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Extra {
    #[allow(dead_code)]
    do_debugging: bool,
    #[allow(dead_code)]
    enable_debugging_extras: bool,
}

impl Default for Basic {
    fn default() -> Self {
        Basic {
            #[allow(dead_code)]
    use_mouse: true,
    #[allow(dead_code)]
    use_keyboard: true,
    #[allow(dead_code)]
    use_controller: false,
    #[allow(dead_code)]
    do_screenshots: true,
    #[allow(dead_code)]
    do_tts: true,
        }
    }
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            #[allow(dead_code)]
    do_debugging: false,
    #[allow(dead_code)]
    enable_debugging_extras: false,
    }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            #[allow(dead_code)]
    basic: Basic::default(),
    #[allow(dead_code)]
    extra: Extra::default()
}}}

#[tokio::main]
async fn main() {
    if Path::new("./config.toml").exists() {
        loop {
        let config_contents = fs::read_to_string("./config.toml").expect("Failed to read TOML file");
        let mut config: Result<Config, Error> = from_str(&config_contents);

        match config {
            Ok(config) => {
                return config;
            }
            Err(err) => {
                println!("{} {} {} {} {}.", Color::Red.paint("[ERR]:"), Color::Fixed(7).paint("The config file has either been"), Color::Red.paint("incorrectly modified"), Color::Fixed(7).paint("or"), Color::Red.paint("had a section removed"));
                println!("{} {}", Color::Blue.paint("[INFO]:"), Color::Cyan.paint("Resetting the config file..."));

                let new_config_contents = to_string_pretty(&Config::default()).expect("Failed to serialize struct to TOML");
                fs::write("./config.toml", new_config_contents).expect("Failed to write updated TOML contents");  
        
                println!("{} {}", Color::Green.paint("[OK]:"), Color::Fixed(30).paint("Sucessfully reset the config file."));      
            }
        }
    }
       } else {
        println!("{} {} {} {} {}.", Color::Purple.paint("[STRANGE]:"), Color::Fixed(7).paint("The config file can't be found, would you like to create one now?"), Color::Red.paint("incorrectly modified"), Color::Fixed(7).paint("or"), Color::Red.paint("had a section removed"));
        println!("{} {}", Color::Blue.paint("[INFO]:"), Color::Cyan.paint("Resetting creating config file..."));

        let new_config_contents = to_string_pretty(&Config::default()).expect("Failed to serialize struct to TOML");
        fs::write("./config.toml", new_config_contents).expect("Failed to write updated TOML contents");  

        println!("{} {}", Color::Green.paint("[OK]:"), Color::Fixed(30).paint("Sucessfully created the config file."));      
       }
}
