use crate::bl_fs::{self, get_entries};
use crate::bl_types;
use crossterm::style::*;
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

// trait FunTrait {
//     fn fun(magic: MagicCrypt256, entry_index: u32, entries: &Vec<bl_types::Entry>)
// }

pub fn bl_use(
    magic: MagicCrypt256,
    verbose: bool,
    fun: &dyn Fn(&MagicCrypt256, u32, &Vec<bl_types::Entry>),
) -> Result<(), std::io::Error> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    writeln!(stdout, "Enter \"qq\" to quit.")?;
    let mut ketamine = false; // dont ask it just popped into my head

    loop {
        let mut count = 0;

        let mut entries = bl_fs::get_entries(&magic);
        for entry in &mut entries {
            count += 1;
            entry.chrono = count;
        }
        if verbose || !ketamine
        /* i couldnt think of anything else to call it ok */
        {
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
                    SetForegroundColor(Color::Reset)
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
                        SetForegroundColor(Color::Reset)
                    )?;
                } else {
                    fun(&magic, num - 1, &entries);
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
                            SetForegroundColor(Color::Reset)
                        )?;
                    }
                }
            }
        }
        ketamine = true;
    }
    Ok(())
}

pub fn del(magic: &MagicCrypt256, entry_index: u32, entries: &Vec<bl_types::Entry>) {
    // let mut stdout = io::stdout();
    let mut entries = entries.to_vec();
    entries.remove(entry_index as usize);
    let mut entries_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(bl_fs::bl_file())
        .unwrap();

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
pub fn read(_: &MagicCrypt256, entry_index: u32, entries: &Vec<bl_types::Entry>) {
    let entry = &entries[entry_index as usize];
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
        SetForegroundColor(Color::Reset)
    )
    .unwrap();
}
pub fn change_password(magic: &MagicCrypt256) {
    // let mut entries_file =

    let mut entries = &bl_fs::get_entries(&magic);

    let mut entries_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(bl_fs::bl_file())
        .unwrap();
    write!(io::stdout(), "Enter your new password: ").unwrap();
    io::stdout().flush();
    let pw = rpassword::read_password().unwrap();
    let mut magic = new_magic_crypt!(pw, 256);
    entries_file.set_len(0).unwrap();

    entries_file
        .write_all(
            magic
                .encrypt_str_to_base64(serde_json::to_string(&entries).unwrap())
                .as_bytes(),
        )
        .unwrap();
}
