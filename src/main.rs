use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let file = File::open(&args[1]);
    let file = match file {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", e);
            }
            _ => {
                panic!("Error opening file: {}", e);
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(l) => println!("{}", l),
            Err(e) => panic!("Error reading line: {}", e),
        }
    }
}
