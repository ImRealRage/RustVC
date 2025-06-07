use std::fs;
use std::io;
use std::path::Path;
use blake3;

pub fn hash_object(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        println!("File does not exist: {}", file_path);
        return Ok(());
    }
    let data = fs::read(path)?;
    // Compute the hash using blake3
    let hash = blake3::hash(&data);
    println!("Hash: {}", hash);
    Ok(())
}
