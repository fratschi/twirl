use anyhow::Result;
use std::env;
use std::fs;
use std::io::Write;
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;

const CODE_FONT: &str = "https://github.com/ryanoasis/nerd-fonts/blob/master/patched-fonts/Iosevka/IosevkaNerdFontMono-Regular.ttf";
const MONOSPACE_FONT: &str = "https://github.com/googlefonts/dm-mono/blob/main/exports/DMMono-Medium.ttf";
const TOML_FILE_TEMPLATE: &[u8] = include_bytes!("twirl.toml");

pub fn check() -> Result<()> {
    let home_dir = home_dir(".twirl")?;
    if !ask_user_to_continue(&home_dir) {
        return Err(Error::new(ErrorKind::Other, "User declined to continue").into());
    }
    init_path(&home_dir)?;
    init_path(&home_dir.join("fonts"))?;
    download_asset(&home_dir.join("fonts/code.ttf"), CODE_FONT)?;
    download_asset(&home_dir.join("fonts/mono.ttf"), MONOSPACE_FONT)?;
    write_toml(&home_dir.join("twirl.toml"))?;
    Ok(())
}

fn init_path(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn home_dir(subdir: &str) -> Result<PathBuf> {
    let dir = env::home_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not determine home directory"))?;
    Ok(dir.join(subdir))
}

fn download_asset(path: &PathBuf, url: &str) -> Result<()> {
    let response = reqwest::blocking::get(url)?;
    let mut file = fs::File::create(&path)?;
    io::copy(&mut response.bytes()?.as_ref(), &mut file)?;
    Ok(())
}

fn write_toml(path: &PathBuf) -> Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(TOML_FILE_TEMPLATE)?;
    Ok(())
}

fn ask_user_to_continue(path: &PathBuf) -> bool {
    if path.exists() {
        return true;
    }
    println!("Do you want to continue to create ~/.twirl and download default fonts? (y/n) ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().eq_ignore_ascii_case("y")
}
