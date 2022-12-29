// import from local module
use io::local_file::LocalFile;

fn main() {
    // create FileRef
    let file = LocalFile::new(".", "test.txt", "test", true, 0o777);

    // create file if it doesn't exist
    file.create(&file.path);

    // write contents to file
    file.write(&file.path, &file.contents);

    // check if file exists
    println!("file exists: {}", file.exists("./test.txt"));
}
