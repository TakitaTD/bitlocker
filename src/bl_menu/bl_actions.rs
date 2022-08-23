use crate::bl_fs;
#[path = "../bl_types.rs"]
mod bl_types;
use crossterm::style::*;
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

// trait FunTrait {
//     fn fun(magic: MagicCrypt256, entry_index: u32, entries: &Vec<bl_types::Entry>)
// }

pub fn bl_use(
    magic: MagicCrypt256,
    verbose: bool,
    fun: &dyn Fn(MagicCrypt256, u32, &Vec<bl_types::Entry>),
) -> Result<(), std::io::Error> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    writeln!(stdout, "Enter \"qq\" to quit.")?;
    let mut ketamine = false; // dont ask it just popped into my head

    loop {
        let mut count = 0;

        let mut entries = OpenOptions::new()
            .read(true)
            .open(bl_fs::bl_file())
            .unwrap();
        let mut entries_str = String::new();
        entries.read_to_string(&mut entries_str)?;
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
        }
        if verbose || !ketamine {
            for entry in &mut entries {
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

        write!(stdout, "Search: ")?;
        let mut buf = String::new();
        stdout.flush()?;
        stdin.read_line(&mut buf)?;
        if buf.trim() == "qq".to_string() {
            break;
        }
        match buf.trim().parse::<u32>() {
            Ok(num) => {
                if num > entries.len() as u32 {
                    writeln!(
                        stdout,
                        "{}error:{} entry out of range",
                        SetForegroundColor(Color::DarkRed),
                        SetForegroundColor(Color::White)
                    )?;
                } else {
                    fun(magic.clone(), num - 1, &entries);
                }
            }
            Err(_) => {
                for entry in &mut entries {
                    if entry.platform.contains(&buf.trim()) || entry.username.contains(&buf.trim())
                    {
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
        ketamine = true;
    }
    Ok(())
}

pub fn del(magic: MagicCrypt256, entry_index: u32, entries: &Vec<bl_types::Entry>) {
    // let mut stdout = io::stdout();
    let mut entries = entries.to_vec();
    entries.remove(entry_index as usize);
    let mut entries_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(bl_fs::bl_file())
        .unwrap();
    // entries_file.seek(SeekFrom::Start(0)).expect("cant seek");
    entries_file.set_len(0).unwrap();
    entries_file
        .write_all(
            magic
                .encrypt_str_to_base64(serde_json::to_string(&entries).unwrap())
                .as_bytes(),
        )
        .unwrap();
    // std::process::exit(0);
}
pub fn read(magic: MagicCrypt256, entry_index: u32, entries: &Vec<bl_types::Entry>) {
    let entry = &entries.to_vec()[entry_index as usize];
    let mut stdout = io::stdout();
    writeln!(
        stdout,
        "{}Platform: {}, Username: {}\nPassword: {}{}",
        SetForegroundColor(Color::Rgb {
            r: 61,
            g: 218,
            b: 174
        }),
        entry.platform,
        entry.username,
        entry.password,
        SetForegroundColor(Color::White)
    )
    .unwrap();
}
