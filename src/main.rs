use std::env;
use std::io;
use std::path;

fn main() {
    let args: Vec<_> = read_vec();
    if args.len() > 1 {
        println!("Usage: lox [script]");
    } else if args.len() == 1 {
        run_file(args[0]);
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &PathBuf) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
}

fn run_prompt() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()

    loop{
        println!("");

        let line = read_vec;
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
