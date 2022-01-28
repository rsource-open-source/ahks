use std::path::Path;

// pub fn find_default_folder() -> Result<bool>
pub fn find_ahk_exe() -> Result<bool, bool> {
    let ahk_exe = "C:\\Program Files\\AutoHotkey\\AutoHotkey.exe";

    if Path::new(ahk_exe).exists() {
        return Ok(true);
    } else {
        return Err(false);
    }
}
