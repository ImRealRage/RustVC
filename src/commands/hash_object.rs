use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use blake3;

/// Write a file's contents to `.rustvc/objects/<hash>`
pub fn hash_object(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        println!("File does not exist: {}", file_path);
        return Ok(());
    }

    let data = fs::read(path)?;
    let hash = blake3::hash(&data);
    let hash_str = hash.to_string();

    // Ensure .rustvc/objects directory exists
    let obj_dir = Path::new(".rustvc/objects");
    fs::create_dir_all(obj_dir)?;

    // Write the data into .rustvc/objects/<hash>
    let mut file = File::create(obj_dir.join(&hash_str))?;
    file.write_all(&data)?;

    println!("Hash: {}", hash_str);
    Ok(())
}
