// import from local module
mod gitignore;

fn main() {
    // check if file exists
    println!("gitignore exists: {}", gitignore::gitignore_exists());
    // add line to file
    println!("added to gitignore: {}", gitignore::add_to_gitignore("target/"));

}
 