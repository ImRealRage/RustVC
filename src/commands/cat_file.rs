use std::fs;
use std::io;
use std::path::Path;

/// Reads and prints contents of .rustvc/objects/<hash>
pub fn cat_file(hash: &str) -> io::Result<()> {
    let object_path = Path::new(".rustvc/objects").join(hash);
    if !object_path.exists() {
        println!("Object not found for hash: {}", hash);
        return Ok(());
    }

    let contents = fs::read_to_string(object_path)?;
    println!("{}", contents);
    Ok(())
}
