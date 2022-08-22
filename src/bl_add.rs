#[path = "./bl_fs.rs"]
mod bl_fs;
#[path = "./bl_types.rs"]
mod bl_types;
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use rpassword;
use std::fs::OpenOptions;
use std::io::{stdin, stdout, Read, Write};
use std::io::{Seek, SeekFrom};

pub fn add(magic: MagicCrypt256) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    let stdin = stdin();
    let mut entry = bl_types::Entry::defaults();
    write!(stdout, "Enter the platform: ")?;
    stdout.flush()?;
    stdin.read_line(&mut entry.platform)?;
    write!(stdout, "Enter your username: ")?;
    stdout.flush()?;
    stdin.read_line(&mut entry.username)?;

    write!(stdout, "Enter your password: ")?;
    stdout.flush()?;
    entry.password = rpassword::read_password().unwrap();

    entry.platform = entry.platform.trim().to_string();
    entry.username = entry.username.trim().to_string();
    entry.password = entry.password.trim().to_string();

    let mut entries_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(bl_fs::bl_file())?;
    let mut entries = String::new();
    entries_file.read_to_string(&mut entries);
    let mut entries = serde_json::from_str::<Vec<bl_types::Entry>>(
        magic
            .decrypt_base64_to_string(entries.as_str())
            .unwrap()
            .as_str(),
    )
    .unwrap_or(vec![]);

    entries.push(entry);

    entries_file.seek(SeekFrom::Start(0))?;
    entries_file.write_all(
        magic
            .encrypt_str_to_base64(serde_json::to_string(&entries).unwrap())
            .as_bytes(),
    )?;

    // writeln!(stdout, "{:?}", magic.encrypt_str_to_base64(entry))?;

    Ok(())
}
