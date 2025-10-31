use std::{fs, io};

fn main() {
    let file_path = "./sample.txt";

    match fs::read_to_string(file_path) {
        Ok(content) => println!("Content: {}", content),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => println!("File {} not found!!!", file_path),
            io::ErrorKind::PermissionDenied => println!("Permission not granted for {}", file_path),
            _ => println!("Some error occured!!!! {}", e),
        },
    }
}
