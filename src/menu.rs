use std::io::{self, Result, Write};

pub fn init_menu() -> Result<()> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    writeln!(stdout, "Hello, World!")?;
    let mut user_input = String::new();
    stdin.read_line(&mut user_input)?;
    writeln!(stdout, "{}", user_input)?;
    Ok(())
}
