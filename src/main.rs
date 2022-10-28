use std::io::Write;
use std::path::Path;

use bitlocker::filesystem::init_filesystem;
use crossterm::{
    style::{Color, SetForegroundColor},
    Result,
};
use std::io;
pub mod filesystem;
pub mod log;
pub mod menu;

fn main() -> Result<()> {
    // using the macro
    let mut stdout = io::stdout();

    writeln!(
        stdout,
        "{}Welcome to Bitlocker v2{}",
        SetForegroundColor(Color::Blue),
        SetForegroundColor(Color::White)
    )?;
    if !Path::new(&filesystem::get_bitlocker_file()).exists() {
        init_filesystem()?;
    }
    loop {
        menu::init_menu()?;
    }
}
