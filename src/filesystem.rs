use crate::log;
use dirs;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rpassword;
use std::{
    fs::{self, OpenOptions},
    io::{self, Result, Write},
    path::Path,
};

fn get_home_dir() -> String {
    return dirs::home_dir()
        .unwrap()
        .as_os_str()
        .to_owned()
        .to_os_string()
        .to_str()
        .unwrap()
        .to_string();
}

pub fn get_bitlocker_dir() -> String {
    return format!("{}/.bitlocker", get_home_dir());
}

pub fn get_bitlocker_file() -> String {
    return format!("{}/data.json", get_bitlocker_dir());
}
pub fn init_filesystem() -> Result<()> {
    let mut stdout = io::stdout();
    log::info("Initializing database...")?;
    if !Path::new(&get_bitlocker_dir()).exists() {
        fs::create_dir_all(get_bitlocker_dir())?;
    }
    writeln!(stdout, "Master password: ")?;
    let password = rpassword::read_password()?;

    if !Path::new(&get_bitlocker_file()).exists() {
        let mut bitlocker_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(get_bitlocker_file())?;
        let magic = new_magic_crypt!(password, 256);
        bitlocker_file.write_all(magic.encrypt_str_to_base64("{}").as_bytes())?;
    }
    Ok(())
}
