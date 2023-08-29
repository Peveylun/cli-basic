use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(args: &[String]) {
    if args.len() < 3 {
        println!("Usage: cli_basic grep <pattern> <filename>");
        return;
    }

    let pattern = &args[2];
    let filename = &args[3];

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Error {}: {}", filename, err);
            return;
        }
    };

    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if line.contains(pattern) {
                println!("{}: {}", line_num + 1, line);
            }
        }
    }
}