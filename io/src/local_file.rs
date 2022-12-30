use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

pub struct LocalFile {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub contents: String,
    pub ephemeral: bool,
    pub permissions: u32,
}

impl LocalFile {
    pub fn new(
        path: &str,
        name: &str,
        contents: &str,
        ephemeral: bool,
        permissions: u32,
    ) -> LocalFile {
        let file = LocalFile {
            path: format!("{}/{}", path, name),
            name: name.to_string(),
            size: 0,
            contents: contents.to_string(),
            ephemeral,
            permissions,
        };

        // We do not want to create a file if it is ephemeral
        if file.ephemeral {
            return file;
        }

        // create file if it doesn't exist
        if !file.exists(&file.path) {
            file.create(&file.path);
            // write contents to file
            file.write(&file.path, contents);
        }

        file
    }

    pub fn exists(&self, path: &str) -> bool {
        let path = Path::new(path);
        path.exists() && path.is_file()
    }

    pub fn create(&self, path: &str) -> File {
        let path = Path::new(path);

        let file = match File::create(path) {
            Err(why) => panic!("couldn't create {}: {}", path.display(), why),
            Ok(file) => file,
        };

        return file;
    }

    pub fn write(&self, path: &str, contents: &str) {
        let mut file = File::create(path).expect("Unable to create file");

        file.write_all(contents.as_bytes())
            .expect("Unable to write data");
    }
}
