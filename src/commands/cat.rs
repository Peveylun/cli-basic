use std::{fs::File, io::Read};

pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: cat <filename>");
        return;
    }

    let filename = &args[0];

    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file {}: {}", filename, err);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file {}: {}", filename, err);
        return;
    }

    println!("{}", contents);
}