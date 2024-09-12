use cached::proc_macro::cached;
#[cfg(not(feature = "invisibility"))]
use dialoguer::{theme::ColorfulTheme, Confirm};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use toml::{de::Error, from_str, to_string_pretty};

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, Hash, PartialEq)]
/// Jerry's config struct.
pub struct Config {
    #[allow(dead_code)]
    basic: Basic,
    #[allow(dead_code)]
    pub extra: Extra,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[allow(clippy::struct_excessive_bools)]
/// Basic options, like whether to enable keyboard inputs or not.
struct Basic {
    #[allow(dead_code)]
    use_mouse: bool,
    #[allow(dead_code)]
    use_keyboard: bool,
    #[allow(dead_code)]
    use_controller: bool,
    #[allow(dead_code)]
    use_pen: bool,
    #[allow(dead_code)]
    do_screenshots: bool,
    #[allow(dead_code)]
    do_tts: bool,
    #[allow(dead_code)]
    do_gen_tts: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Hash, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
/// Extra options, like whether to use an external sentence api or not.
pub struct Extra {
    #[allow(dead_code)]
    pub do_debugging: bool,
    #[allow(dead_code)]
    pub enable_debugging_extras: bool,
    #[allow(dead_code)]
    use_external_sentence_api: bool,
    #[allow(dead_code)]
    no_local_sentence_gen: bool,
}

impl Default for Basic {
    fn default() -> Self {
        Self {
            #[allow(dead_code)]
            use_mouse: true,
            #[allow(dead_code)]
            use_keyboard: true,
            #[allow(dead_code)]
            use_controller: false,
            #[allow(dead_code)]
            use_pen: false,
            #[allow(dead_code)]
            do_screenshots: true,
            #[allow(dead_code)]
            do_tts: true,
            #[allow(dead_code)]
            do_gen_tts: false,
        }
    }
}

#[cached]
/// Attempt to read a config file, or create one.
pub fn get_config() -> Config {
    loop {
        if Path::new("./config.toml").exists() {
            loop {
                let config_contents =
                    fs::read_to_string("./config.toml").expect("Failed to read TOML file");
                let config: Result<Config, Error> = from_str(&config_contents);

                match config {
                    Ok(config) => {
                        return config;
                    }
                    Err(_err) => {
                        println!("The config file has either been incorrectly modified or has had a section removed.");
                        println!("Resetting the config file...");

                        let new_config_contents = to_string_pretty(&Config::default())
                            .expect("Failed to serialize struct to TOML");
                        fs::write("./config.toml", new_config_contents)
                            .expect("Failed to write updated TOML contents");

                        println!("Sucessfully reset the config file.");
                    }
                }
            }
        } else {
            #[cfg(feature = "invisibility")]
            {
                let new_config_contents = to_string_pretty(&Config::default())
                    .expect("Failed to serialize struct to TOML");
                fs::write("./config.toml", new_config_contents)
                    .expect("Failed to write updated TOML contents");
            }
            #[cfg(not(feature = "invisibility"))]
            {
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        "The config file can't be found, would you like to create one now?"
                            .to_string(),
                    )
                    .wait_for_newline(true)
                    .interact()
                    .unwrap()
                {
                    println!("Creating config file...");

                    let new_config_contents = to_string_pretty(&Config::default())
                        .expect("Failed to serialize struct to TOML");
                    fs::write("./config.toml", new_config_contents)
                        .expect("Failed to write updated TOML contents");

                    println!("Sucessfully created the config file.");
                } else {
                    println!("Using default config file.");
                    println!("Using a default config is not recomended. To ignore this prompt and gain more customizability, create a dedicated config file.");
                    return Config::default();
                }
            }
        }
    }
}

/// Get a vector of possible inputs from a Config object.
pub fn get_options(config: &Config) -> Vec<(&'static str, usize)> {
    let mut options: Vec<(&'static str, usize)> = vec![];

    if config.basic.use_mouse {
        options.push(("mouse", 30));
    }
    if config.basic.use_keyboard {
        options.push(("keyboard", 50));
    }
    if config.basic.do_screenshots {
        options.push(("screenshot", 1));
    }
    if config.basic.do_tts {
        options.push(("quote", 10));
    }
    if config.basic.do_gen_tts {
        if config.extra.use_external_sentence_api {
            options.push(("quote_gen_ext", 5));
        }
        if !config.extra.no_local_sentence_gen {
            options.push(("quote_gen", 5));
        }
        if config.extra.no_local_sentence_gen && !config.extra.use_external_sentence_api {
            println!("`do_gen_tts` is active, but no sentence generator is selected. is this supposed to be on?");
        }
    }

    #[cfg(feature = "advanced")]
    {
        if config.basic.use_controller {
            options.push(("gamepad", 50));
        }
        if config.basic.use_pen {
            options.push(("pen", 15));
        }
    }

    options
}
