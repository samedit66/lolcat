use std::env;
use std::fs;
use std::io;

mod color;
mod rainbow;
use rainbow::rainbowize;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() == 0 {
        lolcat_stdin();
    } else {
        for path in &args {
            if path == "-" {
                lolcat_stdin();
            } else {
                lolcat_file(path);
            }
        }
    }
}

fn lolcat_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(text) => print!("{}", rainbowize(&text)),
        Err(e) => eprintln!("Error opening '{path}': {e}"),
    }
}

fn lolcat_stdin() {
    let text = io::stdin()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>()
        .join("\n");
    print!("{}", rainbowize(&text));
}
