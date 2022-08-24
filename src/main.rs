extern crate rpassword;
// use rpassword;
use crossterm::style::*;
mod bl_fs;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
// use std::error::Error;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
mod bl_menu;

fn main() -> Result<(), std::io::Error> {
    let mut stdout = std::io::stdout();
    match Path::new(&bl_fs::bl_file()).exists() {
        true => {
            write!(stdout, "{}", SetForegroundColor(Color::Red))?;
            write!(
                stdout,
                "{}ACCESS DENIED{}\n",
                SetForegroundColor(Color::DarkRed),
                SetForegroundColor(Color::Reset)
            )?;
            let mut magic;

            loop {
                write!(stdout, "Please enter your password: ")?;
                stdout.flush()?;

                let pw = rpassword::read_password()?;

                magic = new_magic_crypt!(pw, 256);

                let mut master_file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .read(true)
                    .open(bl_fs::bl_file())?;
                let mut buf = String::new();
                master_file.read_to_string(&mut buf)?;
                if let Err(_) = magic.decrypt_base64_to_string(&buf) {
                    writeln!(
                        stdout,
                        "{}error:{} wrong password!",
                        SetForegroundColor(Color::Red),
                        SetForegroundColor(Color::Reset)
                    )?;
                    continue;
                }
                break;
            }
            loop {
                bl_menu::init(magic.clone()).unwrap();
            }
        }
        false => {
            write!(
                stdout,
                "{}Set up your master password: ",
                SetForegroundColor(Color::Reset)
            )?;

            stdout.flush()?;
            let pw = rpassword::read_password()?;
            let pw = new_magic_crypt!(pw, 256);

            if !Path::new(&bl_fs::bl_dir()).is_dir() {
                create_dir_all(&bl_fs::bl_dir())?;
            }

            let mut master_file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(bl_fs::bl_file())?;
            master_file.write_all(pw.encrypt_str_to_base64("[]").as_bytes())?;
        }
    }
    Ok(())
}
