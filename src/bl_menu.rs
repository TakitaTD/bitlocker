mod bl_actions;
mod bl_add;
#[path = "./bl_fs.rs"]
mod bl_fs;
#[path = "./bl_types.rs"]
mod bl_types;
use crossterm::style::{Color, ContentStyle, SetForegroundColor, SetStyle};
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use rand::Rng;
use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};
// use std::io::{self, Write};
// extern crate bitlocker;
pub fn init(magic: MagicCrypt256) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    // write!(stdout, "\r");
    write!(stdout, "{esc}[2J{esc}[1;1H", esc = 27 as char);

    let mut entries = OpenOptions::new()
        .read(true)
        .open(bl_fs::bl_file())
        .unwrap();
    let mut entries_str = String::new();
    entries.read_to_string(&mut entries_str);
    let mut entries = serde_json::from_str::<Vec<bl_types::Entry>>(
        magic
            .decrypt_base64_to_string(entries_str)
            .unwrap()
            .as_str(),
    )
    .unwrap();

    // writeln!(stdout, "{:?}", entries)?;
    let mut rng = rand::thread_rng();
    let mut count = 0;
    write!(stdout, "{}", SetForegroundColor(Color::White));

    write!(
        stdout,
        "1. Add a password\n2. Read a password\n3. Delete a password\n"
    )?;
    let user_input;
    loop {
        let mut buf = String::new();
        write!(stdout, "> ")?;
        stdout.flush()?;
        stdin.read_line(&mut buf)?;
        //if buf.trim() == "q".to_string() {
        //    writeln!(stdout, "quitting..")?;
        //    break;
        //}
        match buf.trim().parse::<u8>() {
            Ok(num) => {
                if num > 3 {
                    writeln!(
                        stdout,
                        "{}error:{} number out of range",
                        SetForegroundColor(Color::DarkRed),
                        SetForegroundColor(Color::White)
                    )?;
                    continue;
                }
                user_input = num;
                break;
            }
            Err(err) => {
                if buf.trim() == "q" {
                    writeln!(stdout, "quitting...")?;
                    std::process::exit(0);
                }
                writeln!(
                    stdout,
                    "{}error:{} {err}",
                    SetForegroundColor(Color::DarkRed),
                    SetForegroundColor(Color::White)
                )?;
                continue;
            }
        }
    }
    match user_input {
        1 => bl_add::add(magic).unwrap(),
        2 => bl_actions::bl_use(magic, false, &bl_actions::read).unwrap(),
        3 => bl_actions::bl_use(magic, true, &bl_actions::del).unwrap(),
        _ => writeln!(stdout, "wot").unwrap(),
    };
    Ok(())
}
