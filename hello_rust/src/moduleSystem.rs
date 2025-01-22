pub fn PathItem() {
    // Path & Item
    use std::path::Path;
    use std::fs::File;
    use std::io::Read;

    // Path
    let path = Path::new("hello.txt");
    println!("Path: {:?}", path);

    // File
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    println!("File contents: {:?}", contents);

    // Item
    let item = contents.split_whitespace().next().unwrap();
    println!("Item: {:?}", item);
}

pub fn PackageCreate() {
    // Package
    use std::fs::{File, create_dir};
    use std::io::Write;

    // Create directory
    create_dir("hello").expect("Failed to create directory");

    // Create file
    let mut file = File::create("hello/hello.txt").expect("Failed to create file");
    write!(file, "Hello, world!").expect("Failed to write to file");
}

pub fn Module() {
    // Module
    mod module {
        pub fn hello() {
            println!("Hello, world!");
        }
    }
    module::hello();
}