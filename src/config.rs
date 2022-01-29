use std::path::Path;

pub fn find_file(file: &str) -> Result<bool, bool> {
    if Path::new(file).exists() {
        return Ok(true);
    } else {
        return Err(false);
    }
}
