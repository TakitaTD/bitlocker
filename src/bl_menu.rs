mod bl_actions;
mod bl_add;

#[path = "./bl_fs.rs"]
mod bl_fs;
#[path = "./bl_types.rs"]
mod bl_types;
use crossterm::style::{Color, SetForegroundColor};
use magic_crypt::MagicCrypt256;
// use std::io;
use std::io::{self, Write};
// extern crate bitlocker;
pub fn init(magic: MagicCrypt256) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    write!(stdout, "{}", SetForegroundColor(Color::Reset))?;
    write!(
        stdout,
        "{}",
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )?;
    // write!(stdout, "\x1B[2J\x1B[1;1H")?;
    write!(
        stdout,
        "{}TIP:{} Type \"qq\" at any time to quit!\n1. Add a password\n2. Read a password\n3. Delete a password\n",
        SetForegroundColor(Color::Green),
        SetForegroundColor(Color::Reset),
    )?;
    let user_input;
    loop {
        let mut buf = String::new();
        write!(stdout, "> ")?;
        stdout.flush()?;
        stdin.read_line(&mut buf)?;
        match buf.trim().parse::<u8>() {
            Ok(num) => {
                if num > 3 {
                    writeln!(
                        stdout,
                        "{}error:{} number out of range",
                        SetForegroundColor(Color::DarkRed),
                        SetForegroundColor(Color::Reset)
                    )?;
                    continue;
                }
                user_input = num;
                break;
            }
            Err(err) => {
                if buf.trim() == "qq" {
                    writeln!(stdout, "quitting...")?;
                    std::process::exit(0);
                }
                writeln!(
                    stdout,
                    "{}error:{} {err}",
                    SetForegroundColor(Color::DarkRed),
                    SetForegroundColor(Color::Reset)
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
