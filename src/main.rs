use std::env;
mod commands;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: cli_basic <command> [args]");
        return;
    }

    let comm = &args[1];

    match comm.as_str() {
        "echo" => { commands::echo::run(&args[2..]); }
        "ls" => { commands::ls::run(); }
        "cat" => { commands::cat::run(&args[2..]); }
        "grep" => { commands::grep::run(&args)}
        _ => { println!("Unknown command: {}", comm); }
    }
}   