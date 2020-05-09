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
    let x: i64 = read();
    let mut a = (x / 11) * 2;

    if x % 11 == 0 {
    } else if x % 11 <= 6 {
        a += 1;
    } else {
        a += 2;
    }
    println!("{}", a);
}
