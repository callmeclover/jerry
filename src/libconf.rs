use std::{ fs, path::Path};
use toml::{de::Error, from_str, to_string_pretty};
use ansi_term::Color;
use dialoguer::{theme::ColorfulTheme, Confirm};

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

pub async fn get_config() -> Config {
    loop {
        if Path::new("./config.toml").exists() {
        loop {
        let config_contents = fs::read_to_string("./config.toml").expect("Failed to read TOML file");
        let config: Result<Config, Error> = from_str(&config_contents);

        match config {
            Ok(config) => {
                return config;
            }
            Err(_err) => {
                println!("{} The config file has either been incorrectly modified or has had a section removed.", Color::Red.paint("[ERR]:"));
                println!("{} Resetting the config file...", Color::Blue.paint("[INFO]:"));

                let new_config_contents = to_string_pretty(&Config::default()).expect("Failed to serialize struct to TOML");
                fs::write("./config.toml", new_config_contents).expect("Failed to write updated TOML contents");  
        
                println!("{} Sucessfully reset the config file.", Color::Green.paint("[OK]:"));      
            }
        }
    }
       } else {
        if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("{} The config file can't be found, would you like to create one now?", Color::Purple.paint("[STRANGE]:")))
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        println!("{} Creating config file...", Color::Blue.paint("[INFO]:"));

        let new_config_contents = to_string_pretty(&Config::default()).expect("Failed to serialize struct to TOML");
        fs::write("./config.toml", new_config_contents).expect("Failed to write updated TOML contents");  

        println!("{} Sucessfully created the config file.", Color::Green.paint("[OK]:"));      
    } else {
        println!("{} Using default config file.", Color::Blue.paint("[INFO]:"));
        println!("{} Using a default config is not recomended. To ignore this prompt and gain more customizability, create a dedicated config file.", Color::Yellow.paint("[WARN]:"));      
        return Config::default();
    }
   }
}
}

pub async fn get_options(config: Config) -> Vec<(&'static str, usize)> {
    let mut options: Vec<(&'static str, usize)> = vec![];

    if config.basic.use_mouse {
        options.push(("mouse", 30));
    }
    if config.basic.use_keyboard {
        options.push(("keyboard", 50));
    }
    if config.basic.use_controller {
        options.push(("gamepad", 50));
    }
    if config.basic.do_screenshots {
        options.push(("screenshot", 5));
    }
    if config.basic.do_tts {
        options.push(("quote", 15));
    }

    return options;
}