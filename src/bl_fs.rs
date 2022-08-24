use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    pub custom_dir: Option<String>,
}
use dirs;

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
