use std::fs;
use std::path::Path;
use std::io;

pub fn init() -> io::Result<()> {
    let rustvc_dir = Path::new(".rustvc");
    if rustvc_dir.exists() {
        println!("Repository already initialized.");
    } else {
        fs::create_dir(rustvc_dir)?;
        println!("Initialized empty RustVC repository in .rustvc/");
    }
    Ok(())
}
