mod fileOps;

pub fn gitignore_exists() -> bool {
    fileOps::file_exists(".gitignore")
}

pub fn add_to_gitignore(line: &str) -> bool {
    if !fileOps::line_exists_in_file(".gitignore", line) {
        fileOps::add_line_to_file(".gitignore", line);
        return true
    }
    false
}
