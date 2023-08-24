use std::fs;

pub fn run() {
    if let Ok(ent) = fs::read_dir(".") {
        for i in ent {
            if let Ok(i) = i {
                println!("{}", i.file_name().to_string_lossy());
            }
        }
    }
}