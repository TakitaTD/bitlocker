use clap::Parser;
// #[path = "bl_types.rs"]
// mod bl_types;
use crate::bl_types;
use std::fs::OpenOptions;
use std::io::{Read, Write};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    pub custom_dir: Option<String>,
}
use dirs;
use magic_crypt::{MagicCrypt256, MagicCryptTrait};

pub fn bl_file() -> String {
    return format!("{}/pw.json", bl_dir());
}
pub fn bl_dir() -> String {
    match Args::parse().custom_dir {
        Some(dir) => {
            format!("{}/.bitlocker", dir)
        }
        None => {
            format!(
                "{}/.bitlocker",
                dirs::home_dir()
                    .expect("unable to get home directory")
                    .as_os_str()
                    .to_string_lossy()
                    .to_string()
            )
        }
    }
}
pub fn get_entries(magic: &MagicCrypt256) -> Vec<bl_types::Entry> {
    let mut entries = OpenOptions::new().read(true).open(bl_file()).unwrap();
    let mut entries_str = String::new();
    entries.read_to_string(&mut entries_str);
    let mut entries = serde_json::from_str::<Vec<bl_types::Entry>>(
        magic
            .decrypt_base64_to_string(entries_str)
            .unwrap()
            .as_str(),
    )
    .unwrap();
    return entries;
}
