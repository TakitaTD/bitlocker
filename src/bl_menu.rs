#[path = "./bl_add.rs"]
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
        2 => {
            for entry in &mut entries {
                count += 1;
                entry.chrono = count;
                writeln!(
                    stdout,
                    "{}. {}Platform: {}, Username: {}{}",
                    entry.chrono,
                    SetForegroundColor(Color::Rgb {
                        r: 219,
                        g: 87,
                        b: 120
                    }),
                    entry.platform,
                    entry.username,
                    SetForegroundColor(Color::White)
                )?;
            }
            writeln!(stdout, "Enter \"qq\" to quit.")?;
            loop {
                write!(stdout, "Search: ")?;
                let mut buf = String::new();
                stdout.flush()?;
                stdin.read_line(&mut buf)?;
                if buf.trim() == "qq".to_string() {
                    writeln!(stdout, "quitting...")?;
                    break;
                }
                match buf.trim().parse::<u32>() {
                    Ok(num) => {
                        let entry = &entries[(num - 1) as usize];
                        writeln!(stdout, "Platform: {}\nUsername: {}\nPassword: {}", entry.platform, entry.username, entry.password)?;
                    }
                    Err(_) => {
                        let mut count = 0;
                        for entry in &mut entries {
                            if entry.platform.contains(&buf.trim())
                                || entry.username.contains(&buf.trim())
                            {
                                count += 1;

                                writeln!(
                                    stdout,
                                    "{}. {}Platform: {}, Username: {}{}",
                                    entry.chrono,
                                    SetForegroundColor(Color::Rgb {
                                        r: 219,
                                        g: 87,
                                        b: 120
                                    }),
                                    entry.platform,
                                    entry.username,
                                    SetForegroundColor(Color::White)
                                )?;
                            }
                        }
                    }
                }
            }
            init(magic);
        }
        _ => writeln!(stdout, "wot").unwrap(),
    };
    Ok(())
}
