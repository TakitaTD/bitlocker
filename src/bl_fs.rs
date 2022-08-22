use dirs;

pub fn bl_file() -> String {
    return format!(
        "{}/.bitlocker/pw.json",
        dirs::home_dir()
            .expect("unable to get home directory")
            .as_os_str()
            .to_string_lossy()
            .to_string()
    );
}
pub fn bl_dir() -> String {
    return format!(
        "{}/.bitlocker",
        dirs::home_dir()
            .expect("unable to get home directory")
            .as_os_str()
            .to_string_lossy()
            .to_string()
    );
}
