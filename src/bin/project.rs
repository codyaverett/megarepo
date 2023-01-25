use std::io::Read;
use toml::Table;

// function to read a file
fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = read_file("Cargo.toml")?;
    let toml = contents.parse::<Table>()?;

    let package = toml.get("package").unwrap().as_table().unwrap();
    let name = package.get("name").unwrap().as_str().unwrap();
    let version = package.get("version").unwrap().as_str().unwrap();

    println!("{} {}", name, version);

    Ok(())
}
