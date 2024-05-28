#[allow(unused_imports)]
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::{fs, path::Path};
use toml::{de::Error, from_str, to_string_pretty};
use cached::{proc_macro::cached, SizedCache};
#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone, Eq, Hash, PartialEq)]
pub struct Config {
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    basic: Basic,
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    extra: Extra,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone, Hash, PartialEq, Eq)]
struct Extra {
    #[allow(dead_code)]
    do_debugging: bool,
    #[allow(dead_code)]
    enable_debugging_extras: bool,
    #[allow(dead_code)]
    use_external_sentence_api: bool,
    #[allow(dead_code)]
    no_local_sentence_gen: bool,
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

#[cached(
    ty = "SizedCache<String, Config>",
    create = "{ SizedCache::with_size(1) }",
)]
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

pub async fn get_options(config: Config) -> Vec<(&'static str, usize)> {
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
