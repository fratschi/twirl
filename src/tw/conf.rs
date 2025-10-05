use anyhow::Result;
use std::fs;
use std::io::{Error, ErrorKind};
use toml::{Table, Value};

use crate::tw;

#[derive(Debug, Clone)]
pub struct TwirlConfig {
    pub font : FontConfig,
    pub themes: Theme,
}

impl Default for TwirlConfig {
    fn default() -> Self {
        TwirlConfig {
            font: FontConfig::default(),
            themes: Theme::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FontConfig {
    pub code: String,
    pub mono: String,
}

impl Default for FontConfig {
    fn default() -> Self {
        FontConfig {
            code: "code.ttf".to_string(),
            mono: "mono.ttf".to_string(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Theme {
    pub background: String,
    pub foreground: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            background: "#181818".to_string(),
            foreground: "#efefef".to_string(),
            red: "#bf0000".to_string(),
            green: "#00bf00".to_string(),
            yellow: "#bfbf00".to_string(),
        }
    }
}

pub fn load_config() -> Result<TwirlConfig> {
    let config_path = tw::init::home_dir(".twirl/twirl.toml")?;
    if !config_path.exists() {
        return Err(Error::new(ErrorKind::NotFound, "Configuration file not found").into());
    }
    let config_content = fs::read_to_string(config_path)?;
    let value: Table = config_content.parse::<Table>()?;

    let mut cfg = TwirlConfig::default();

    for (section, contents) in value.iter() {
        match section.as_str() {
            "fonts" => match contents {
                Value::Table(s) => {
                    for (key, value) in s.iter() {
                        match key.as_str() {
                            "code" => cfg.font.code = get(value),
                            "mono" => cfg.font.mono = get(value),
                            _ => continue,
                        }
                    }
                },
                _ => continue,
            },

            // TODO extend to support more themes with subsections
            "theme" =>  match contents {        
                Value::Table(s) => {
                     println!("Found theme section: {}", section);
                    for (key, value) in s.iter() {
                        match key.as_str() {
                            "background" => cfg.themes.background = get(value),
                            "foreground" => cfg.themes.foreground = get(value),
                            "red" => cfg.themes.red = get(value),
                            "green" => cfg.themes.green = get(value),
                            "yellow" => cfg.themes.yellow =  get(value),
                            _ => continue,                            
                        }
                    }                   
                },
                _   => continue,
            },
            _ => continue,
        }
    }

    Ok(cfg)
}


fn get(value: &Value) -> String {

    match value {
        Value::String(s) => s.clone(),
        Value::Integer(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        _ => "".to_string(),
    }
}

