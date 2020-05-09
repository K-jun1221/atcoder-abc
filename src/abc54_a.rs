use std::io::*;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let a: usize = read();
    let b: usize = read();
    if a == b {
        println!("Draw");
        return;
    }

    if a == 1 {
        println!("Alice");
        return;
    }

    if b == 1 {
        println!("Bob");
        return;
    }

    if a > b {
        println!("Alice")
    } else {
        println!("Bob")
    }
}
