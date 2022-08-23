#[path = "bl_add.rs"]
mod bl_add;
use crate::bl_fs;
#[path = "../bl_types.rs"]
mod bl_types;
use crossterm::style::{Color, ContentStyle, SetForegroundColor, SetStyle};
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use rand::Rng;
use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};
pub fn read(magic: MagicCrypt256) -> Result<(), std::io::Error> {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();
    let mut count = 0;
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
                if (num > entries.len() as u32) {
                    writeln!(
                        stdout,
                        "{}error:{} entry out of range",
                        SetForegroundColor(Color::DarkRed),
                        SetForegroundColor(Color::White)
                    );
                } else {
                    let entry = &entries[(num - 1) as usize];
                    writeln!(
                        stdout,
                        "{}Platform: {}, Username: {}\nPassword: {}{}",
                        SetForegroundColor(Color::Rgb {
                            r: 219,
                            g: 87,
                            b: 120
                        }),
                        entry.platform,
                        entry.username,
                        entry.password,
                        SetForegroundColor(Color::White)
                    )?;
                }
            }
            Err(_) => {
                let mut count = 0;
                for entry in &mut entries {
                    if entry.platform.contains(&buf.trim()) || entry.username.contains(&buf.trim())
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
    Ok(())
}
