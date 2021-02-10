
use std::fs;
use std::io::Result;

pub fn write_file(filename: String, content: String) -> Result<()> {
    fs::write(filename, content)?;
    Ok(())
}