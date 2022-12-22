// file related functions

use std::{path::Path, fs::{File, OpenOptions}, io::{Write, Read}};

// function to check if file exists
pub fn file_exists(path: &str) -> bool {
    let path = Path::new(path);
    path.exists() && path.is_file()
}

// function to read file
pub fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

// function to write file
pub fn write_file(path: &str, contents: &str) {
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(contents.as_bytes())
        .expect("Unable to write data");
}

// function to append line to file
pub fn append_line_to_file(path: &str, line: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("Unable to open file");
    file.write_all(line.as_bytes())
        .expect("Unable to write data");
}

// function to check if line exists in file
pub fn line_exists_in_file(path: &str, line: &str) -> bool {
    let contents = read_file(path);
    contents.contains(line)
}

// function to add line to file
pub fn add_line_to_file(path: &str, line: &str) {
    if line_exists_in_file(path, line) {
        return
    } 
    
    // append line to file
    append_line_to_file(path, line);
}
