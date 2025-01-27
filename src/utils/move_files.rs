use std::fs;
use std::path::{Path, PathBuf};

pub fn move_file(from: PathBuf, to: &Path) -> Result<(), String> {
    let file_name = from
        .file_name()
        .ok_or_else(|| "Failed to get file name".to_string())?;

    let to_path = to.join(file_name);

    fs::rename(&from, &to_path).map_err(|e| format!("Failed to move file: {}", e))?;
    
    println!("{} moved to {}", from.display(), to_path.display());
    Ok(())
}
