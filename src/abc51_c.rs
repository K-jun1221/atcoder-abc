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
    let sx: i64 = read();
    let sy: i64 = read();
    let tx: i64 = read();
    let ty: i64 = read();
    let mut ans: String = String::new();
    let x: usize = (tx - sx) as usize;
    let y: usize = (ty - sy) as usize;

    for _ in 0..x {
        ans += &"R".to_string();
    }
    for _ in 0..y {
        ans += &"U".to_string();
    }
    for _ in 0..x {
        ans += &"L".to_string();
    }
    for _ in 0..y {
        ans += &"D".to_string();
    }
    ans += "D";
    for _ in 0..x + 1 {
        ans += &"R".to_string();
    }
    for _ in 0..y + 1 {
        ans += &"U".to_string();
    }

    ans += "L";
    ans += "U";

    for _ in 0..x + 1 {
        ans += &"L".to_string();
    }
    for _ in 0..y + 1 {
        ans += &"D".to_string();
    }

    ans += "R";
    println!("{}", ans);
}
