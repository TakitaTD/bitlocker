use crossterm::style::{Color, SetForegroundColor};
use std::io::{self, Result, Write};

pub fn info(msg: &str) -> Result<()> {
    let mut stdout = io::stdout();
    writeln!(
        stdout,
        "{}[info]{} {}",
        SetForegroundColor(Color::Green),
        SetForegroundColor(Color::White),
        msg
    )?;
    Ok(())
}
