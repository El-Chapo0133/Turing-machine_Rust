
use std::fs;


pub fn read_file(filename: String) -> std::io::Result<String> {
    fs::read_to_string(filename)
}