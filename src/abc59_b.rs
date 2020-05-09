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
    let a: String = read();
    let b: String = read();
    let ac: Vec<char> = a.chars().collect();
    let bc: Vec<char> = b.chars().collect();
    let mut done = false;

    if ac.len() > bc.len() {
        println!("GREATER");
        done = true;
    }

    if bc.len() > ac.len() {
        println!("LESS");
        done = true;
    }
    if done {
        return;
    }

    for i in 0..ac.len() {
        let ai = ac[i].to_digit(10).unwrap();
        let bi = bc[i].to_digit(10).unwrap();
        if ai > bi {
            println!("GREATER");
            done = true;
            break;
        }

        if bi > ai {
            println!("LESS");
            done = true;
            break;
        }
    }
    if !done {
        println!("EQUAL")
    }
}
